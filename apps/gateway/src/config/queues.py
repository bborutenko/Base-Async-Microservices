import json
from typing import AsyncGenerator

from aiokafka import AIOKafkaProducer

from config.settings import settings as st


async def get_producer() -> AsyncGenerator[AIOKafkaProducer]:
    producer = AIOKafkaProducer(
        bootstrap_servers=st.kafka_bootstrap_servers,
        value_serializer=lambda v: json.dumps(v).encode("utf-8"),
    )
    await producer.start()
    try:
        yield producer
    finally:
        await producer.stop()
