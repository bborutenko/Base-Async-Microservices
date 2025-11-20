from uuid import UUID

from aiokafka import AIOKafkaProducer
from fastapi import APIRouter, Depends, Request, status

from config.queues import get_producer
from shared.analytics import AnalyticsService
from shared.utils import get_correlation_id

from . import schemas as sch
from .service import OrdersService

router = APIRouter(tags=["Orders"], prefix="/order")


@router.post(
    "",
    description="Endpoint for creating order.",
    status_code=status.HTTP_201_CREATED,
)
async def create_order(
    order: sch.CreateOrder,
    request: Request,
    producer: AIOKafkaProducer = Depends(get_producer),
    correlation_id: UUID = Depends(get_correlation_id),
) -> sch.DisplayOrder:
    created_order = await OrdersService.create_order(producer, order)
    await AnalyticsService.create_order(
        producer, correlation_id, created_order, request
    )

    return created_order
