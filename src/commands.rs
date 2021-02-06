use crate::config::{Config};
use crate::producer::Producer;
use crate::commands_schema::{Command, Value};
use crate::inputs_schema::{ ValueInput };

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
    initial_value: ValueInput,
    producer: Producer,
    config: Config
) -> Result<impl warp::Reply, Infallible> {

    info!("Received {:#?}", initial_value);

    let producer = producer.lock().await;
    let command_id = Uuid::new_v4();
    let value_id = Uuid::new_v4();
    let command = Command::CREATE_VALUE { id: command_id,
                                          data: Value { id : value_id,
                                                        value : initial_value.value }};

    let payload : String = serde_json::to_string(&command).expect ("Could not serialize command");

    match producer.send(FutureRecord::to(&config.commands_topic)
                        .payload(&payload)
                        .key(&format!("{}", &command_id)),
                        Duration::from_secs(0)).await {
        Ok(_) => {
            info!("Succesfully sent command {:#?} to topic {}", command, &config.commands_topic)
        },
        Err(why) => {
            warn!("Error sending command: {:#?}", why)
        },
    };

    Ok(warp::reply::json(&value_id))
}

// TODO
// pub async fn update_value(
//     id: Uuid,
//     operation : Operation,
//     producer: Producer,
//     config: Config
// ) -> Result<impl warp::Reply, Infallible> {

//     info!("Update value {:#?} with {:#?}", id, operation);


//     Ok(StatusCode::NOT_FOUND)
// }
