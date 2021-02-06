use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{debug, info, warn, error};
use crate::config::{Config};

pub type Producer = Arc<Mutex<FutureProducer>>;

pub fn init(config : &Config) -> Producer {

    let (_, version) = get_rdkafka_version();
    info!("librdkafka version: {}", version);

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &config.broker)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    Arc::new(Mutex::new(producer))
}