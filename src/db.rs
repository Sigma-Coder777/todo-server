use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::{Error, Surreal};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
    pub id: Option<Thing>,
    pub title: String,
    pub is_done: bool,
}
impl Todo {
    pub fn new(title: String) -> Self {
        Todo {
            id: None,
            title,
            is_done: false,
        }
    }
    pub fn to_json(&self) -> serde_json::Value {
        json!(
            {
                "id":self.id.as_ref().unwrap().id.to_string(),
                "title":self.title,
                "is_done":self.is_done
            }
        )
    }
}
pub struct DataBase {
    table: String,
    db: Surreal<Client>,
}

pub async fn get_database() -> Result<Surreal<Client>, Error> {
    let db = Surreal::new::<Ws>("0.0.0.0:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("todo-db").use_db("todos").await?;
    Ok(db)
}
impl DataBase {
    pub async fn new() -> Self {
        Self {
            db: get_database().await.unwrap(),
            table: String::from("todos"),
        }
    }
    pub async fn get_all_todo(&self) -> Result<Vec<Todo>, Error> {
        let records = self.db.select(&self.table).await?;
        Ok(records)
    }
    pub async fn new_todo(&self, todo_title: String) -> Result<Vec<Todo>, Error> {
        let record = self
            .db
            .create(&self.table)
            .content(Todo::new(todo_title))
            .await?;
        Ok(record)
    }

    pub async fn get_todo_by_id(&self, id: String) -> Result<Option<Todo>, Error> {
        let thing_id = Thing {
            tb: self.table.clone(),
            id: id.into(),
        };

        let record: Option<Todo> = self.db.select(thing_id).await?;

        Ok(record)
    }

    pub async fn toggle_todo_by_id(&self, id: String) -> Result<Option<Todo>, Error> {
        if let Some(mut todo) = self.get_todo_by_id(id.clone()).await? {
            todo.is_done = !todo.is_done;
            let updated_todo = self.db.update((&self.table, id)).content(&todo).await?;
            Ok(updated_todo)
        } else {
            Ok(None)
        }
    }

    pub async fn remove_todo_by_id(&self, id: String) -> Result<Option<Todo>, Error> {
        let removed_todo = self.db.delete((&self.table, id.clone())).await?;
        Ok(removed_todo)
    }
    pub async fn clear_all_todos(&self) -> Result<Vec<Todo>, Error> {
        let cleared = self.db.delete(&self.table).await?;
        Ok(cleared)
    }
}
