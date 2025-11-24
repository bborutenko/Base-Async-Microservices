use rdkafka::config::ClientConfig;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::stream_consumer::StreamConsumer;

use crate::config::consumer::start;

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

    pub async fn run(&self, kafka_connection_attempts: &i32) {
        start(*kafka_connection_attempts, &self.consumer).await;
    }
}
