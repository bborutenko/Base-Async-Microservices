"""
FastAPI application entrypoint for the Gateway service.

Exposes `app` for ASGI servers (e.g., uvicorn, hypercorn).
Routers are registered via `shared.routers.include_routers`.

Health endpoints provided by shared routers:
- GET /api/health
"""

import asyncio

import uvicorn
from aiokafka import AIOKafkaProducer
from fastapi import FastAPI

from config.settings import settings as st
from orders.router import router as o_router
from shared.logging import get_logger, setup_logging
from shared.routers import router as h_router


async def check_kafka_connectivity() -> None:
    """
    Try to connect to Kafka on startup; fail fast if not reachable.
    """
    producer = AIOKafkaProducer(bootstrap_servers=st.kafka_bootstrap_servers)
    try:
        await producer.start()
    finally:
        await producer.stop()


setup_logging(st.log_level)
logger = get_logger()

app = FastAPI(
    title="Gateway",
    version="0.1.0",
    docs_url="/api/docs",
    redoc_url="/api/redoc",
    openapi_url="/api/openapi.json",
)

app.include_router(h_router)
app.include_router(o_router)

if __name__ == "__main__":
    try:
        asyncio.run(check_kafka_connectivity())
    except Exception as e:
        logger.fatal("Kafka connectivity check failed: %s", e)
        raise SystemExit(1)

    uvicorn.run(
        "main:app",
        host=st.host,
        port=st.port,
        reload=st.reload,
    )
