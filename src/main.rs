mod schema;
mod commands;
mod commander;
mod utils;
mod config;

use log::{debug, info, warn, error};
use std::env;
use std::thread;
use std::time::{Instant, Duration};
use tokio::runtime::Runtime;
use tokio::time;
use config::{Config, Load};

// use rdkafka::client::ClientContext;
// use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
// use rdkafka::consumer::stream_consumer::StreamConsumer;
// use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
// use rdkafka::error::KafkaResult;
// use rdkafka::message::{Headers, Message};
// use rdkafka::topic_partition_list::{TopicPartitionList, Offset};
// use rdkafka::util::{get_rdkafka_version, Timeout} ;
// use std::collections::HashMap;
// use std::convert::TryInto;
// use futures::future::lazy;
// use commander;

// The event handler:
// - subscribes to the event log (a Kafka topic)
// - consumes and processes events
// - applies the resulting updates to the read store

// #[derive(Debug, Clone)]
// struct Config {
//     log_level: String,
//     topic: String,
//     broker: String,
//     group_id: String
// }

// struct CustomContext;

// impl ClientContext for CustomContext {}

// impl ConsumerContext for CustomContext {
//     fn commit_callback(&self, result: KafkaResult<()>, offsets: &TopicPartitionList) {
//         debug!("Committing offsets {:?}", offsets);
//     }
// }

// async fn handle_event (config : &Config) {

//     let Config { group_id, broker, topic, ..} = config;
//     let context = CustomContext;

//     let consumer: StreamConsumer<CustomContext> = ClientConfig::new()
//         .set("group.id", group_id)
//         .set("bootstrap.servers", broker)
//         .set("enable.partition.eof", "false")
//         .set("session.timeout.ms", "6000")
//         .set("enable.auto.commit", "false")
//         .set_log_level(RDKafkaLogLevel::Debug)
//         .create_with_context(context)
//         .expect("Consumer creation failed");

//     consumer
//         .subscribe(&[topic])
//         .expect("Can't subscribe to specified topics");

//     // set offset for replaying
//     let mut topic_map: HashMap<(String, i32), Offset> = HashMap::new ();
//     topic_map.insert((String::from (topic), 0), Offset::Beginning);
//     let tpl : TopicPartitionList = TopicPartitionList::from_topic_map (&topic_map).unwrap ();
//     consumer.assign (&tpl);

//     loop {
//         match consumer.recv().await {
//             Err(why) => error!("Failed to read message: {}", why),
//             Ok(m) => {

//                 // TODO : read payload (avro)

//                 info!("key: '{:?}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
//                       m.key(), m.topic(), m.partition(), m.offset(), m.timestamp());

//                 // TODO : store offset per payload type

//                 match consumer.commit_message(&m, CommitMode::Async) {
//                     Err(why) => error!("Failed to comit message offset: {}", why),
//                     Ok (_) => {
//                         info!("Commited message offset: {}", m.offset ());
//                     }
//                 };

//             }

//         };
//     }

// }

fn main() {

    // let config = Config {};

    let config : Config = Config::load();


    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Create the runtime
    let rt  = Runtime::new().unwrap ();

    // Spawn the root task
    rt.block_on(async {

        let t1 = tokio::spawn(async {
            commander::run ().await;
        });

        let t2 = tokio::spawn(async {
            let mut interval = time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                info!("ping2 from: {:#?}", thread::current().id());

            }
        });

        t1.await.expect ("Ooops!");
        t2.await.expect ("Ooops!");
    });
}

// #[tokio::main]
// async fn main() {

//     let config = Config {
//         log_level: get_env_var ("LOG_LEVEL", Some (String::from ("info"))),
//         topic: get_env_var ("KAFKA_TOPICS", Some (String::from ("events-stream"))),
//         broker: get_env_var ("KAFKA_BROKER", Some (String::from ("localhost:9092"))),
//         group_id: get_env_var ("KAFKA_CONSUMER_GROUP_ID", Some (String::from ("event_consumers"))),
//     };

//     env::set_var("RUST_LOG", &config.log_level);
//     env_logger::init();

//     let (_, version_s) = get_rdkafka_version();
//     info!("librdkafka version: {}", version_s);
//     info!("{:#?}", &config);

//     // TODO : ensure topics

//     handle_event(&config).await
// }

// fn get_env_var (var : &str, default: Option<String> ) -> String {
//     match env::var(var) {
//         Ok (v) => v,
//         Err (_) => {
//             match default {
//                 None => panic! ("Missing ENV variable: {} not defined in environment", var),
//                 Some (d) => d
//             }
//         }
//     }
// }

// fn string_to_vector (s : &str) -> Vec<String> {
//     s.split(",").map(|x| String::from (x)).collect()
// }

// pub fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
