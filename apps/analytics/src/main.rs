pub mod config;
pub mod handler;

use tokio;

use config::app::App;
use config::logging::setup_logging;
use config::settings::Settings;

#[tokio::main]
async fn main() {
    setup_logging();

    let settings = Settings::new();

    let app = App::new(
        settings.kafka_bootstrap_servers(),
        settings.kafka_group_id(),
        settings.kafka_input_topic(),
    );
    app.run(settings.kafka_connection_attempts()).await;
}
