use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::{TopicPartitionList, Offset};
use rdkafka::util::{get_rdkafka_version, Timeout} ;
use log::{debug, info, warn, error};
use std::collections::HashMap;
use crate::config::{Config, Load};
use std::sync::Arc;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn commit_callback(&self, result: KafkaResult<()>, offsets: &TopicPartitionList) {
        debug!("Committing offsets {:?}", offsets);
    }
}

pub async fn run (config : Arc<Config>) {

    let Config { broker, commands_group_id, commands_topic, ..} = &*config;
    let context = CustomContext;

    let consumer: StreamConsumer<CustomContext> = ClientConfig::new()
        .set("group.id", commands_group_id)
        .set("bootstrap.servers", broker)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&commands_topic])
        .expect("Can't subscribe to specified topics");

    // set offset for replaying
    let mut topic_map: HashMap<(String, i32), Offset> = HashMap::new ();
    topic_map.insert((String::from (commands_topic), 0), Offset::Beginning);
    let tpl : TopicPartitionList = TopicPartitionList::from_topic_map (&topic_map).unwrap ();
    consumer.assign (&tpl);

    loop {
        match consumer.recv().await {
            Err(why) => error!("Failed to read message: {}", why),
            Ok(m) => {

                // TODO : read payload (avro)

                info!("key: '{:?}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), m.topic(), m.partition(), m.offset(), m.timestamp());

                // TODO : store offset per payload type

                match consumer.commit_message(&m, CommitMode::Async) {
                    Err(why) => error!("Failed to comit message offset: {}", why),
                    Ok (_) => {
                        info!("Commited message offset: {}", m.offset ());
                    }
                };

            }

        };
    }

}
