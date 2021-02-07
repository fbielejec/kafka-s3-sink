mod command_processor;
mod commander;
mod commands;
mod commands_schema;
mod config;
mod inputs_schema;
mod producer;
mod events_schema;

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

    // Spawn the root task
    rt.block_on(async {

        let config_rc1 = Arc::clone(&config);
        let t1 = tokio::spawn(async {
            commander::run (config_rc1).await;
        });

        let config_rc2 = Arc::clone(&config);
        let t2 = tokio::spawn(async {
            command_processor::run (config_rc2).await;
        });

        // TODO : materialized view

        t1.await.expect ("Ooops!");
        t2.await.expect ("Ooops!");
    });
}
