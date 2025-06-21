use crate::{AppState, model::TaskModel, schema::CreateTaskSchema};
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{Data, Json, ServiceConfig, scope},
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

#[post("/task")]
async fn create_task_handler(body: Json<CreateTaskSchema>, data: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(
        TaskModel,
        "INSERT INTO tasks (title, content) VALUES ($1, $2) RETURNING *",
        body.title,
        body.content
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(task) => {
            let task_response = json!({
                "status": "success",
                "data": task
            });
            return HttpResponse::Ok().json(task_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            }));
        }
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(health_checker_handler)
        .service(create_task_handler);
    cfg.service(scope);
}
