use log::{debug, info, warn, error};
use std::env;

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub log_level: String,
    pub commands_topic: String,
    pub commands_group_id: String,
    pub broker: String,
}

pub trait Load {
    // Static method signature; `Self` refers to the implementor type
    fn load() -> Self;
}

impl Load for Config {
    fn load() -> Config {
        Config {
            log_level: get_env_var ("LOG_LEVEL", Some (String::from ("info"))),
            commands_topic: get_env_var ("KAFKA_COMMANDS_TOPICS", Some (String::from ("commands"))),
            commands_group_id: get_env_var ("KAFKA_COMMANDS_GROUP_ID", Some (String::from ("commands-processors"))),
            broker: get_env_var ("KAFKA_BROKER", Some (String::from ("localhost:9092"))),
        }
    }
}

fn get_env_var (var : &str, default: Option<String> ) -> String {
    match env::var(var) {
        Ok (v) => v,
        Err (_) => {
            match default {
                None => panic! ("Missing ENV variable: {} not defined in environment", var),
                Some (d) => d
            }
        }
    }
}
