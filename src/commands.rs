use crate::schema::{Value, Operation};
use log::{debug, info, warn, error};
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use std::convert::Infallible;
use uuid::Uuid;
use warp::http::StatusCode;

// TODO : publish a command to commands topic

pub async fn create_value(
    value: Value,
    // db: Db,
) -> Result<impl warp::Reply, Infallible> {

    info!("Create value {:#?}", value);

    let uuid = Uuid::new_v4();

    // TODO
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "brokers")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");



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
