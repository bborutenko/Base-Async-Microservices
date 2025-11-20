from datetime import datetime
from uuid import UUID

from aiokafka import AIOKafkaProducer

from shared.logging import get_logger

from .schemas import CreateOrder, DisplayOrder

logger = get_logger()


class OrdersService:
    @classmethod
    async def create_order(
        cls, producer: AIOKafkaProducer, order: CreateOrder, correlation_id: UUID
    ) -> DisplayOrder:
        logger.info("Sending create order to queue", order.__dict__)
        created_order = DisplayOrder(
            id="ord_0001",
            product_id=order.product_id,
            quantity=order.quantity,
            price=order.price,
            currency=order.currency,
            user_id=order.user_id,
            status="created",
            created_at=datetime.now(),
            updated_at=datetime.now(),
        )
        # TODO: use `producer` to publish event to Kafka if required
        return created_order
