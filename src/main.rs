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

fn main() {

    let config : Config = Config::load();

    env::set_var("RUST_LOG", &config.log_level);
    env_logger::init();

    info!("{:#?}", &config);

    // Create the runtime
    let rt = Runtime::new().unwrap ();

    // Spawn the root task
    rt.block_on(async {
        let t1 = tokio::spawn(async {
            // let config = config.clone ();
            commander::run ( config).await;
        });

        let t2 = tokio::spawn(async {
            // let mut interval = time::interval(Duration::from_secs(10));
            // loop {
            //     interval.tick().await;
            //     info!("ping2 from: {:#?}", thread::current().id());

            // }

            // command_processor::run (Box::new (config)).await;

        });

        t1.await.expect ("Ooops!");
        t2.await.expect ("Ooops!");
    });
}
