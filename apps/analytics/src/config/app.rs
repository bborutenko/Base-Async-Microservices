use futures::TryStreamExt;
use uuid::Uuid;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use tokio::time::Instant;
use tracing::{Instrument, error, info, info_span};

use crate::analytics::handler::process_message;

pub struct App {
    consumer: StreamConsumer,
}

impl App {
    pub fn new(brokers: &String, group_id: &String, input_topic: &String) -> App {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("allow.auto.create.topics", "true")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(&[&input_topic])
            .expect("Can't subscribe to specified topic");

        App { consumer }
    }

    pub async fn run(&self) {
        let stream_processor = self
            .consumer
            .stream()
            .try_for_each(|borrowed_message| async move {
                let owned = borrowed_message.detach();
                let msg_id = Uuid::new_v4();
                let span = info_span!("process_message", msg_id = %msg_id);
                let started = Instant::now();
                tokio::spawn(async move {
                    let result = async {
                        info!("start processing message");
                        process_message(owned).await
                    }
                    .instrument(span)
                    .await;
                    match result {
                        Ok(_) => info!(duration_ms = started.elapsed().as_millis(), "message processed successfully"),
                        Err(error) => error!(duration_ms = started.elapsed().as_millis(), error = %error, "message processing failed"),
                    };
                });
                Ok(())
            });

        info!("starting event loop");
        stream_processor.await.expect("stream processing failed");
        info!("stream processing terminated");
    }
}
