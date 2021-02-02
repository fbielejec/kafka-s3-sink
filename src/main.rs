use log::{debug, info, warn, error};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;
use std::env;
use std::convert::TryInto;


// The event handler:
// - subscribes to the event log (a Kafka topic)
// - consumes and processes events
// - applies the resulting updates to the read store

#[derive(Debug, Clone)]
struct Config {
    log_level: String,
    topics: Vec<String>,
    broker: String,
    group_id: String
}

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

async fn handle_event (config : &Config) {

    let Config { group_id, broker, topics, ..} = config;
    let context = CustomContext;

    let consumer: StreamConsumer<CustomContext> = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", broker)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    let t: Vec<&str> = topics.iter().map(|x| x.as_str ()).collect();

    consumer
        .subscribe(&t)
        .expect("Can't subscribe to specified topics");

}

#[tokio::main]
async fn main() {

    let config = Config {
        log_level: get_env_var ("LOG_LEVEL", Some (String::from ("info"))),
        topics: string_to_vector (&get_env_var ("KAFKA_TOPICS", Some (String::from ("events")))),
        broker: get_env_var ("KAFKA_BROKER", Some (String::from ("localhost:9092"))),
        group_id: get_env_var ("KAFKA_CONSUMER_GROUP_ID", Some (String::from ("event_consumers"))),
    };

    env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();

    let (_, version_s) = get_rdkafka_version();
    info!("librdkafka version: {}", version_s);
    info!("{:#?}", &config);

    handle_event(&config).await
}

fn get_env_var (var : &str, default: Option<String> ) -> String {
    match env::var(var) {
        Ok (v) => v,
        Err (_) => {
            match default {
                None => panic! ("Missing ENV variable: {} not defined in environment", var),
                Some (d) => d
            }
        }
    }
}

fn string_to_vector (s : &str) -> Vec<String> {
    s.split(",").map(|x| String::from (x)).collect()
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
