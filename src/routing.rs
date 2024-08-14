use std::sync::Arc;

use axum::routing::{get, patch, post};
use axum::{Extension, Json, Router};
use serde_json::{json, Value};
use tracing::{debug, error, info};

use crate::db::{DataBase, Todo};

fn todos_to_json(todos: Vec<Todo>) -> Value {
    json!(todos
        .iter()
        .map(|todo| todo.to_json())
        .collect::<Vec<Value>>())
}

async fn list_todos(Extension(db): Extension<Arc<DataBase>>) -> Json<serde_json::Value> {
    info!("Fetching all todos");
    let mytodos: Vec<Todo> = db.get_all_todo().await.unwrap();
    debug!("All todos: {:?}", mytodos);
    Json(todos_to_json(mytodos))
}

async fn add_todo(
    Extension(db): Extension<Arc<DataBase>>,
    todo_title: String,
) -> Json<serde_json::Value> {
    info!("Adding new todo: {}", todo_title);
    let newtodo = db.new_todo(todo_title).await.unwrap()[0].clone();
    debug!("Added Todo {:?}", newtodo);

    Json(newtodo.to_json())
}

async fn toggle_todo(
    Extension(db): Extension<Arc<DataBase>>,
    todo_id: String,
) -> Json<serde_json::Value> {
    info!("Toggle  todo: {}", todo_id);
    let newtodo = db.toggle_todo_by_id(todo_id).await;
    match newtodo {
        Ok(Some(updated_todo)) => {
            debug!("Toggled Todo {:?}", &updated_todo);
            Json(updated_todo.to_json())
        }
        Ok(None) => Json(json!("Todo not found with given id")),

        Err(e) => {
            error!("Error Toggling Todo : {:?}", e);
            Json(json!("No Todo Found"))
        }
    }
}

async fn remove_todo(
    Extension(db): Extension<Arc<DataBase>>,
    todo_id: String,
) -> Json<serde_json::Value> {
    info!("Removing  todo: {}", todo_id);
    let newtodo = db.remove_todo_by_id(todo_id).await;
    match newtodo {
        Ok(Some(updated_todo)) => {
            debug!("Removed Todo {:?}", &updated_todo);
            Json(updated_todo.to_json())
        }
        Ok(None) => Json(json!("Todo not found with given id")),

        Err(e) => {
            error!("Error Removing Todo : {:?}", e);
            Json(json!("Something went wrong"))
        }
    }
}
pub async fn clear_todos(Extension(db): Extension<Arc<DataBase>>) -> Json<serde_json::Value> {
    info!("Clearing all the todos");
    match db.clear_all_todos().await {
        Ok(todos) => {
            debug!("Deleted all todos");
            Json(todos_to_json(todos))
        }
        Err(e) => {
            error!("Something went wrong clearing todos {:?}", e);
            Json(json!("Error clearing all todos"))
        }
    }
}

pub async fn run_server(port: String) {
    println!("Initializing Database");
    let db = Arc::new(DataBase::new().await);

    let app = Router::new()
        .route("/list", get(list_todos))
        .route("/add", post(add_todo))
        .route("/toggle", patch(toggle_todo))
        .route("/remove", post(remove_todo))
        .route("/clear", post(clear_todos))
        .layer(Extension(db));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();
    info!("Server is running on 0.0.0.0:{}", port);
    axum::serve(listener, app).await.unwrap();
}
