use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub product_id: String,
    pub user_email: String,
    pub quantity: u32,
    pub price: f64,
    pub currency: String,
    pub user_id: String,
}
