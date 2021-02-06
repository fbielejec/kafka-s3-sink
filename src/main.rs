mod schema;
mod commands;
mod commander;
mod utils;
mod config;
mod producer;
mod command_processor;

use log::{debug, info, warn, error};
use std::env;
use std::thread;
use std::time::{Instant, Duration};
use tokio::runtime::Runtime;
use tokio::time;
use config::{Config, Load};
use std::boxed::Box;
use std::sync::Arc;


fn main() {

    let config = Arc::new (Config::load());

    env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();

    info!("{:#?}", &config);

    // Create the runtime
    let rt = Runtime::new().unwrap ();

    // Spawn the root task
    rt.block_on(async  {

        let config_rc1 = Arc::clone(&config);
        let t1 = tokio::spawn(async move {
            commander::run ( config_rc1 ).await;
        });

        let config_rc2 = Arc::clone(&config);
        let t2 = tokio::spawn(async move {
            command_processor::run ( config_rc2 ).await;

        });

        t1.await.expect ("Ooops!");
        t2.await.expect ("Ooops!");
    });
}
