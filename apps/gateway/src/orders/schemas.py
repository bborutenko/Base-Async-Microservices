from datetime import datetime

from pydantic import BaseModel


class CreateOrder(BaseModel):
    product_id: str
    quantity: int
    price: float
    currency: str
    user_id: str


class DisplayOrder(BaseModel):
    id: str
    product_id: str
    quantity: int
    price: float
    currency: str
    user_id: str
    status: str
    created_at: datetime
    updated_at: datetime
