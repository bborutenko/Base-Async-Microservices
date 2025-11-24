use futures::TryStreamExt;
use uuid::Uuid;

use rdkafka::consumer::stream_consumer::StreamConsumer;
use tokio::time::Instant;
use tracing::{Instrument, error, info, info_span, warn};

use crate::core::handler::process_message;

pub async fn start(attempts: i32, consumer: &StreamConsumer) {
    let mut attempt: i32 = 0;

    while attempt <= attempts {
        let stream_processor = consumer
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
        let stream_result = stream_processor.await;
        match stream_result {
            Ok(_) => {
                info!("connected");
            }
            Err(err) => {
                if attempt + 1 < 10 {
                    warn!("connect failed: {:?}; retrying", err);
                    attempt += 1;
                } else {
                    error!("connect failed after 10 attempts: {:?}", err);
                    panic!("cannot connect to Kafka");
                }
            }
        }
    }
}
