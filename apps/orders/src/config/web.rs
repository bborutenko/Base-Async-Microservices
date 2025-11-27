use actix_web::{App, HttpServer, web};

use crate::core::router as rtr;
use crate::core::state::AppState;

pub async fn start(host: String, port: u16, app_state: web::Data<AppState>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::scope("/order").route("/", web::post().to(rtr::create_order)))
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
