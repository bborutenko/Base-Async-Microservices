use rdkafka::message::OwnedMessage;

pub async fn process_message(message: OwnedMessage) -> std::io::Result<()> {
    println!("processing message {:?}", message);
    Ok(())
}
