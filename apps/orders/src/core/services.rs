use crate::core::schemas as sch;

pub struct OrderService {}

impl OrderService {
    pub async fn create_order(order: sch::Order) -> sch::Order {
        order
    }
}
