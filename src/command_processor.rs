use crate::commands_schema::ActionType::{CREATE_VALUE, UPDATE_VALUE};
use crate::commands_schema::{Command, Value, UpdateOperation};
use crate::config::{Config, Load};
use crate::events_schema::Event;
use crate::producer::Producer;
use crate::producer;
use log::{debug, info, warn, error};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::OwnedHeaders;
use rdkafka::message::{Headers, Message};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::topic_partition_list::{TopicPartitionList, Offset};
use rdkafka::util::{get_rdkafka_version, Timeout} ;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn commit_callback(&self, result: KafkaResult<()>, offsets: &TopicPartitionList) {
        debug!("Committing offsets {:?}", offsets);
    }
}

pub async fn run (config : Arc<Config>) {

    let Config { broker, commands_group_id, commands_topic, .. } = &*config;
    let context = CustomContext;

    let mut state = HashMap::<Uuid, f64>::new();
    let producer = producer::init (&config);
    let consumer: StreamConsumer<CustomContext> = ClientConfig::new()
        .set("group.id", commands_group_id)
        .set("bootstrap.servers", broker)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer.subscribe(&[&commands_topic])
        .expect("Can't subscribe to specified topic");

    // set offset for replaying
    let mut topic_map: HashMap<(String, i32), Offset> = HashMap::new ();
    topic_map.insert((String::from (commands_topic), 0), Offset::Beginning);
    let tpl : TopicPartitionList = TopicPartitionList::from_topic_map (&topic_map).unwrap ();
    consumer.assign (&tpl);

    loop {

        // TODO : store offset

        match consumer.recv().await {
            Err(why) => error!("Failed to read message: {}", why),
            Ok(m) => {
                match m.payload_view::<str>() {
                    None => {
                        warn!("Empty command payload");
                    },
                    Some(Ok(payload)) => {

                        info!("payload: {}", payload);

                        // TODO : read avro
                        match serde_json::from_str::<Command>(payload) {
                            Ok (command) => {
                                info!("Received command: {:?}, partition: {}, offset: {}, timestamp: {:?}", command, m.partition(), m.offset(), m.timestamp());

                                // run validation and emit events
                                match command {
                                    Command::CREATE_VALUE{id, data} => validate_create_value (id, data, &config.clone (), &mut state, producer.clone ()).await,
                                    Command::UPDATE_VALUE {id, data} => validate_update_value (id, data, &config.clone (), &mut state, producer.clone ()).await,
                                };
                            },
                            Err (why) => {
                                error!("Could not deserialize command: {:?}", why);
                            }
                        };

                    },
                    Some(Err(e)) => {
                        error!("Error while deserializing command payload: {:?}", e);
                    }
                };

                match consumer.commit_message(&m, CommitMode::Async) {
                    Err(why) => error!("Failed to commit message offset: {}", why),
                    Ok (_) => {
                        debug!("Commited message offset: {}", m.offset ());
                    }
                };

            }
        };

    }
}

/// implements business logic
async fn validate_create_value (
    command_id: Uuid,
    data : Value,
    config: &Config,
    state : &mut HashMap<Uuid, f64>,
    producer : Producer
) {

    info!("validating command: {:?}", data);

    let value_id = data.value_id;
    match state.contains_key(&value_id) {
        true => {
            error!("command {} rejected: value with id {} already exists", command_id, value_id);
        },
        false => {
            // emit event
            let event_id = Uuid::new_v4();
            let event = Event::VALUE_CREATED { id: event_id,
                                               parent: command_id,
                                               data: data.clone () };
            let payload : String = serde_json::to_string(&event).expect ("Could not serialize event");
            let producer = producer.lock().await;

            match producer.send(FutureRecord::to(&config.events_topic)
                                .payload(&payload)
                                .key(&format!("{}", &event_id)),
                                Duration::from_secs(0)).await {
                Ok(_) => {
                    info!("Succesfully sent event {:#?} to topic {}", event, &config.events_topic);
                    state.insert (command_id, data.value);
                },
                Err(why) => warn!("Error sending event: {:#?}", why)
            };
        }
    }
}

/// implements business logic
async fn validate_update_value (
    command_id: Uuid,
    data : UpdateOperation,
    config: &Config,
    state : &mut HashMap<Uuid, f64>,
    producer : Producer
) {

    info!("validating command: {:?}", data);

    let value_id = data.value_id;
    match state.contains_key(&value_id) {
        false => {
            error!("command {} rejected: value with id {} does not exist", command_id, value_id);
        },
        true => {
            // emit event
            let event_id = Uuid::new_v4();
            let event = Event::VALUE_UPDATED { id: event_id,
                                               parent: command_id,
                                               data: data.clone () };

            let payload : String = serde_json::to_string(&event).expect ("Could not serialize event");
            let producer = producer.lock().await;

            match producer.send(FutureRecord::to(&config.events_topic)
                                .payload(&payload)
                                .key(&format!("{}", &event_id)),
                                Duration::from_secs(0)).await {
                Ok(_) => info!("Succesfully sent event {:#?} to topic {}", event, &config.events_topic),
                Err(why) => warn!("Error sending event: {:#?}", why),
            };
        }
    }
}
