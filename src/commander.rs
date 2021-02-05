use crate::commands;
use crate::utils::print_type_of;
use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;
use warp::http::StatusCode;
use uuid::Uuid;
use log::{debug, info, warn, error};

// writes to commands topic
// enforces schema

pub async fn run () {

    let routes = create_value()
        .or(update_value());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

/// POST /values {"value" : 2 }
fn create_value() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("values")
        .and(warp::post())
        .and(warp::body::json())
        // .and(with_db(db))
        .and_then(commands::create_value)
}

/// PUT /values/:id {"operation" : "add", "value" : 2 }
fn update_value() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("values" / Uuid)
        .and(warp::put())
        .and(warp::body::json())
        // .and(with_db(db))
        .and_then(commands::update_value)
}
