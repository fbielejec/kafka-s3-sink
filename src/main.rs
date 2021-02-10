mod api;
mod command_processor;
mod commands;
mod commands_schema;
mod config;
mod consumer;
mod db;
mod events_schema;
mod inputs_schema;
mod materialized_view;
mod producer;
mod queries;

use config::{Config, Load};
use log::info;
use std::env;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn main() {

    let config = Arc::new (Config::load());

    env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();

    info!("{:#?}", &config);

    // Create the runtime
    let rt = Runtime::new().unwrap ();
    let db = db::init ();
    // Spawn the root task
    rt.block_on(async {

        let db_rc1 = Arc::clone (&db);
        let config_rc1 = Arc::clone(&config);
        let t1 = tokio::spawn(async {
            api::run (config_rc1, db_rc1).await;
        });

        let config_rc2 = Arc::clone(&config);
        let t2 = tokio::spawn(async {
            command_processor::run (config_rc2).await;
        });

        let config_rc3 = Arc::clone(&config);
        let db_rc2 = Arc::clone (&db);
        let t3 = tokio::spawn(async {
            materialized_view::run (config_rc3, db_rc2).await;
        });

        t1.await.expect ("Ooops!");
        t2.await.expect ("Ooops!");
        t3.await.expect ("Ooops!");
    });
}
