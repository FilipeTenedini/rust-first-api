use actix_web::{
    HttpResponse, Responder, get,
    web::{ServiceConfig, scope},
};
use serde_json::json;

#[get("/health-checker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Hello from API. I'm UP!";

    return HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }));
}
pub fn config(cfg: &mut ServiceConfig) {
    let scope = scope("/api").service(health_checker_handler);
    cfg.service(scope);
}
