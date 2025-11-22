from dataclasses import dataclass
from dataclasses import field as dataclass_field
from datetime import datetime
from typing import Any, Dict, Optional
from uuid import UUID

from aiokafka import AIOKafkaProducer
from fastapi import Request, status

from config.settings import settings as st
from orders.schemas import DisplayOrder

from .logging import get_logger

logger = get_logger()


@dataclass(slots=True)
class AnalyticMessage:
    event_name: str
    correlation_id: UUID
    user_id: Optional[str] = None
    session_id: Optional[str] = None
    timestamp: datetime = dataclass_field(default_factory=datetime.now)
    source: Optional[str] = None
    client_ip: Optional[str] = None
    user_agent: Optional[str] = None
    path: Optional[str] = None
    method: Optional[str] = None
    status_code: Optional[int] = None
    duration_ms: Optional[float] = None
    order_id: Optional[str] = None
    amount: Optional[float] = None
    currency: Optional[str] = None
    metadata: Dict[str, Any] = dataclass_field(default_factory=dict)

    def to_dict(self) -> dict:
        return {
            "event_name": self.event_name,
            "correlation_id": str(self.correlation_id),
            "user_id": self.user_id,
            "session_id": self.session_id,
            "timestamp": self.timestamp.isoformat(),
            "source": self.source,
            "client_ip": self.client_ip,
            "user_agent": self.user_agent,
            "path": self.path,
            "method": self.method,
            "status_code": self.status_code,
            "duration_ms": self.duration_ms,
            "order_id": self.order_id,
            "amount": self.amount,
            "currency": self.currency,
            "metadata": self.metadata,
        }


class AnalyticsService:
    @classmethod
    async def send_message(
        cls, producer: AIOKafkaProducer, message: AnalyticMessage
    ) -> None:
        logger.info("Sending analytics message into queue", message.to_dict())
        await producer.send_and_wait(st.kafka_analytics_output_topic, message.to_dict())

    @classmethod
    async def create_order(
        cls,
        producer: AIOKafkaProducer,
        correlation_id: UUID,
        order: DisplayOrder,
        request: Request,
    ) -> None:
        message = AnalyticMessage(
            event_name="order.created",
            correlation_id=correlation_id,
            user_id=order.user_id,
            session_id=request.cookies.get("sessionid"),
            source="gateway",
            client_ip=request.headers.get("x-forwarded-for", "").split(",")[0].strip()
            or (request.client.host if request.client else None),
            user_agent=request.headers.get("user-agent"),
            path=request.url.path,
            method=request.method,
            status_code=getattr(request.state, "status_code", status.HTTP_201_CREATED),
            duration_ms=getattr(request.state, "duration_ms", None),
            order_id=order.id,
            amount=order.price,
            currency=order.currency,
            metadata={"product_id": order.product_id, "quantity": order.quantity},
        )
        await cls.send_message(producer, message)
