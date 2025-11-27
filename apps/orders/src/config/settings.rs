use std::env::var;
use std::str::FromStr;

use dotenvy::dotenv;
use tracing::debug;

const DEFAULT_APP_HOST: &str = "0.0.0.0";
const DEFAULT_APP_PORT: u16 = 8000;
const DEFAULT_KAFKA_BOOTSTRAP_SERVERS: &str = "127.0.0.1:9202";
const DEFAULT_KAFKA_CONNECTION_ATTEMPTS: i32 = 10;

pub struct Settings {
    app_host: String,
    app_port: u16,
    kafka_bootstrap_servers: String,
    kafka_connection_attempts: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Settings {
        dotenv().ok();

        let app_host = env_or_default("APP_HOST", DEFAULT_APP_HOST);
        let app_port = env_or_default("APP_PORT", DEFAULT_APP_PORT);
        let kafka_bootstrap_servers =
            env_or_default("KAFKA_BOOTSTRAP_SERVERS", DEFAULT_KAFKA_BOOTSTRAP_SERVERS);
        let kafka_connection_attempts = env_or_default(
            "KAFKA_CONNECTION_ATTEMPTS",
            DEFAULT_KAFKA_CONNECTION_ATTEMPTS,
        );

        debug!(
            KAFKA_BOOTSTRAP_SERVERS = kafka_bootstrap_servers,
            "Taking enviroments"
        );

        Settings {
            app_host,
            app_port,
            kafka_bootstrap_servers,
            kafka_connection_attempts,
        }
    }

    pub fn kafka_bootstrap_servers(&self) -> &String {
        &self.kafka_bootstrap_servers
    }

    pub fn app_host(&self) -> &String {
        &self.app_host
    }

    pub fn app_port(&self) -> &u16 {
        &self.app_port
    }

    pub fn kafka_connection_attempts(&self) -> &i32 {
        &self.kafka_connection_attempts
    }
}

fn env_or_default<D, T>(key: &str, default: D) -> T
where
    T: FromStr,
    D: Into<T>,
{
    var(key)
        .ok()
        .and_then(|s| s.parse::<T>().ok())
        .unwrap_or_else(|| default.into())
}
