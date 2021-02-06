use crate::config::{Config};
use crate::producer::Producer;
use crate::commands_schema::{CreateValueCommand, ActionType, Value};
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

    // info!("Received {:#?}", value);

    // let value_id = Uuid::new_v4();

    let producer = producer.lock().await;

    let command = CreateValueCommand { id: Uuid::new_v4(),
                                       action: ActionType::CREATE_VALUE,
                                       data: Value { id : Uuid::new_v4(),
                                                     value : initial_value.value }

                                       // data: hashmap!{String::from ("value") => format! ("{}", value.value)}

    };

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

    Ok(warp::reply::json(&command.data.id))
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
