use std::env::var;

use dotenvy::dotenv;
use tracing::debug;

const DEFAULT_KAFKA_BOOTSTRAP_SERVERS: &str = "127.0.0.1:9202";
const DEFAULT_KAFKA_GROUP_ID: &str = "analytics-consumer";
const DEFAULT_KAFKA_INPUT_TOPIC: &str = "events";

pub struct Settings {
    kafka_bootstrap_servers: String,
    kafka_group_id: String,
    kafka_input_topic: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Settings {
        dotenv().ok();

        let kafka_bootstrap_servers =
            env_or_default("KAFKA_BOOTSTRAP_SERVERS", DEFAULT_KAFKA_BOOTSTRAP_SERVERS);
        let kafka_group_id = env_or_default("KAFKA_GROUP_ID", DEFAULT_KAFKA_GROUP_ID);
        let kafka_input_topic = env_or_default("KAFKA_INPUT_TOPIC", DEFAULT_KAFKA_INPUT_TOPIC);

        debug!(
            KAFKA_BOOTSTRAP_SERVERS = kafka_bootstrap_servers,
            KAFKA_GROUP_ID = kafka_group_id,
            KAFKA_INPUT_TOPIC = kafka_input_topic,
            "Taking enviroments"
        );

        Settings {
            kafka_bootstrap_servers,
            kafka_group_id,
            kafka_input_topic,
        }
    }

    pub fn kafka_bootstrap_servers(&self) -> &String {
        &self.kafka_bootstrap_servers
    }

    pub fn kafka_group_id(&self) -> &String {
        &self.kafka_group_id
    }

    pub fn kafka_input_topic(&self) -> &String {
        &self.kafka_input_topic
    }
}

fn env_or_default(key: &str, default: &str) -> String {
    var(key).unwrap_or_else(|_| default.to_string())
}
