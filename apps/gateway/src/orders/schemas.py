from datetime import datetime
from uuid import UUID

from pydantic import BaseModel


class CreateOrder(BaseModel):
    product_id: str
    user_email: str
    quantity: int
    price: float
    currency: str
    user_id: str


class DisplayOrder(BaseModel):
    id: UUID
    product_id: str
    user_email: str
    quantity: int
    price: float
    currency: str
    user_id: str
    status: str
    created_at: datetime
    updated_at: datetime
