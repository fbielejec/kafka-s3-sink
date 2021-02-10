use crate::commands_schema::{Command, Value, UpdateOperation};
use crate::config::{Config};
use crate::db::Db;
use crate::db;
use crate::inputs_schema::{ ValueInput, ValueOperationInput };
use crate::producer::Producer;
use log::{info, warn};
use rdkafka::producer::FutureRecord;
use serde_json;
use std::convert::Infallible;
use std::time::Duration;
use uuid::Uuid;
use warp::http::StatusCode;

pub async fn get_value(
    value_id: Uuid,
    db: Db
) -> Result<impl warp::Reply, Infallible> {

    info!("Querying value id {} ", value_id);

    match db::get (&db, &value_id).await {
        None => Ok(warp::reply::with_status(warp::reply::json (&format!("No value with id {} exists", &value_id)),
                                            warp::http::StatusCode::NO_CONTENT)),
        Some (value) => {
            let response = Value {value_id : value_id,
                                  value : value};
            Ok(warp::reply::with_status(warp::reply::json(&response),
                                        warp::http::StatusCode::ACCEPTED))
        }

    }

}
