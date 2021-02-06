use crate::config::{Config};
use crate::producer::Producer;
use crate::schema::{Value, Operation, Command, ActionType};
use log::{debug, info, warn, error};
use maplit::hashmap;
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::convert::Infallible;
use uuid::Uuid;
use warp::http::StatusCode;
use serde_json;
use std::time::Duration;

// TODO : serialize as avro
pub async fn create_value(
    value: Value,
    producer: Producer,
    config: Config
) -> Result<impl warp::Reply, Infallible> {

    info!("Received {:#?}", value);

    let uuid = Uuid::new_v4();

    let producer = producer.lock().await;

    let command = Command { id: uuid,
                            action: ActionType::CREATE_VALUE,
                            data: hashmap!{String::from ("value") => format! ("{}", value.value)} };

    let payload : String = serde_json::to_string(&command).expect ("Could not serialize command");

    match producer.send(FutureRecord::to(&config.commands_topic)
                        .payload(&payload)
                        .key(&format!("{}", &command.id)),
                        Duration::from_secs(0)).await {
        Ok(_) => {
            info!("Succesfully sent command {:#?} to topic {}", command, &config.commands_topic)
        },
        Err(why) => {
            warn!("Error sending command: {:#?}", why)
        },
    };

    Ok(warp::reply::json(&uuid))
}

// TODO
pub async fn update_value(
    id: Uuid,
    operation : Operation,
    producer: Producer,
    config: Config
) -> Result<impl warp::Reply, Infallible> {

    info!("Update value {:#?} with {:#?}", id, operation);


    Ok(StatusCode::NOT_FOUND)
}
