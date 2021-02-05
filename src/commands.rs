use crate::config::{Config};
use crate::producer::Producer;
use crate::schema::{Value, Operation};
use log::{debug, info, warn, error};
use std::convert::Infallible;
use uuid::Uuid;
use warp::http::StatusCode;

// TODO : publish a command to commands topic

pub async fn create_value(
    value: Value,
    producer: Producer,
    config: Config)
    -> Result<impl warp::Reply, Infallible> {

    info!("Create value {:#?}", value);

    let uuid = Uuid::new_v4();

    // producer
    //     .send(
    //         FutureRecord::to(topic_name)
    //             .payload(&format!("Message {}", i))
    //             .key(&format!("Key {}", i))
    //             // .headers(OwnedHeaders::new().add("header_key", "header_value")),
    //         Duration::from_secs(0),
    //     )
    //     .await;


    Ok(warp::reply::json(&uuid))
}

pub async fn update_value(
    id: Uuid,
    operation : Operation
    // db: Db,
) -> Result<impl warp::Reply, Infallible> {

    info!("Update value {:#?} with {:#?}", id, operation);

    // let mut customers = db.lock().await;

    // for customer in customers.iter_mut() {
    //     if customer.guid == guid {
    //         *customer = updated_customer;
    //         return Ok(StatusCode::OK);
    //     }
    // }

    Ok(StatusCode::NOT_FOUND)
}
