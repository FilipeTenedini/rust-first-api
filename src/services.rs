use crate::{
    AppState,
    model::TaskModel,
    schema::{CreateTaskSchema, FilterOptions},
};
use actix_web::{
    HttpResponse, Responder, delete, get, post,
    web::{Data, Json, Path, Query, ServiceConfig, scope},
};
use serde_json::json;
use uuid::Uuid;

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

#[get("task/{id}")]
async fn get_task_by_id_handler(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let task = sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks WHERE id = $1",
        path.into_inner(),
    )
    .fetch_one(&data.db)
    .await;

    match task {
        Ok(task) => {
            return HttpResponse::Ok().json(json!({"status": "success", "data": task}));
        }
        Err(e) => {
            return HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            }));
        }
    }
}

#[get("/task")]
async fn get_all_tasks_handler(
    filters: Query<FilterOptions>,
    data: Data<AppState>,
) -> impl Responder {
    let limit: u32 = filters.limit.unwrap_or(10);
    let offset: u32 = (filters.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(tasks) => {
            return HttpResponse::Ok().json(json!({"status": "success", "data": tasks}));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            }));
        }
    }
}

#[delete("task/{id}")]
async fn delete_task_handler(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(
        TaskModel,
        "DELETE FROM tasks WHERE id = $1",
        path.into_inner(),
    )
    .execute(&data.db)
    .await;

    match result {
        Ok(_) => {
            return HttpResponse::Ok().json(json!({"status": "success"}));
        }
        Err(e) => {
            return HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            }));
        }
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(health_checker_handler)
        .service(create_task_handler)
        .service(get_all_tasks_handler)
        .service(get_task_by_id_handler)
        .service(delete_task_handler);

    cfg.service(scope);
}
