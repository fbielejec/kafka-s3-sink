use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;
use std::env;
use log::{debug, info, warn, error};

// The event handler:
// - subscribes to the event log (a Kafka topic)
// - consumes and processes events
// - applies the resulting updates to the read store

#[derive(Debug, Clone)]
struct Config {
    log_level: String,
    topics: Vec<String>,
}

#[tokio::main]
async fn main() {

    let config = Config {
        log_level: get_env_var ("LOG_LEVEL", Some (String::from ("info"))),
        topics: string_to_vector (&get_env_var ("TOPICS", Some (String::from ("events"))))
    };

    env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();


    let (_, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: {}", version_s);
    info!("{:#?}", &config);




    // info!("Done!");
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
    s.split(",").map(|x| x.parse::<String>().unwrap()).collect()
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
