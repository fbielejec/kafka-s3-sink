use crate::config::{Config, Load};
use crate::producer::Producer;
use crate::producer;
use crate::commands_schema::{Command, Value};
use crate::commands_schema::ActionType::CREATE_VALUE;
use crate::commands_schema::ActionType::UPDATE_VALUE;
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
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};


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

    let mut db = HashMap::<Uuid, f64>::new();
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
        // TODO : validate against state db
        // TODO : emit events

        match consumer.recv().await {
            Err(why) => error!("Failed to read message: {}", why),
            Ok(m) => {
                match m.payload_view::<str>() {
                    None => {
                        warn!("Empty command payload");
                    },
                    Some(Ok(payload)) => {

                        info!("payload: {}", payload);

                        // TODO : match command trait
                        // TODO : run validation

                        // TODO : read avro
                        match serde_json::from_str::<Command>(payload) {
                            Ok (command) => {
                                info!("Received command: {:?}, partition: {}, offset: {}, timestamp: {:?}", command, m.partition(), m.offset(), m.timestamp());
                                // validate (&command, &mut db, producer.clone ());

                                match command {
                                    Command::CREATE_VALUE{id, data} => validate_create_value (id, data, &mut db, producer.clone ()),
                                    Command::UPDATE_VALUE {..} => {

                                        warn!("TODO");

                                    },
                                };

                            },
                            Err (why) => {
                                error!("Could not deserialize command: {:?}", why);
                                // TODO : emit command_rejected
                            }
                        };


                    },
                    Some(Err(e)) => {
                        error!("Error while deserializing command payload: {:?}", e);
                        // TODO : emit command_rejected
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

// TODO

// /// implements business logic
fn validate_create_value (
    id: Uuid,
    data : Value,
    db : &mut HashMap<Uuid, f64>,
    producer : Producer
) {

    // command

}
