use actix_web::{HttpResponse, Responder, web};

use crate::core::schemas as sch;
use crate::core::services::OrderService;

pub async fn create_order(order: web::Json<sch::Order>) -> impl Responder {
    let created_order = OrderService::create_order(order.into_inner()).await;
    HttpResponse::Created().json(created_order)
}
