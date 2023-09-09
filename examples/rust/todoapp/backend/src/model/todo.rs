#![allow(unused)]

use sqlx::{Execute, Postgres, QueryBuilder};

use super::db::Db;
use crate::{model, security::UserCtx};

// region: Todo Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
  pub id: i64,
  pub cid: i64, // create id
  pub title: String,
  pub status: TodoStatus,
}

#[derive(Default, Clone)]
pub struct TodoPatch {
  pub title: Option<String>,
  pub status: Option<TodoStatus>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "TODO_STATUS_ENUM")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
  Open,
  Close,
}
// endregion: Todo Types

// region: Todo Model Access Controler
pub struct TodoMac;

impl TodoMac {
  const TABLE: &'static str = "todo";
  const RETURNING: &'static str = "id, cid, title, status";
  const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "status"];
}

impl TodoMac {
  // let mut fields = data.fields();
  // fields.push()
  pub async fn create(db: &Db, utx: &UserCtx, data: TodoPatch) -> Result<Todo, model::Error> {
    let cid: i64 = 123;
    let sql =
      "INSERT INTO todo (cid, title, status) VALUES ($1, $2, $3) RETURNING id, cid, title, status";
    let query = sqlx::query_as::<_, Todo>(&sql)
      .bind(cid) // TODO: Should come from user context
      .bind(data.title.unwrap_or_else(|| "untitled".to_string()))
      .bind(TodoStatus::Open);
    let todo = query.fetch_one(db).await?;
    Ok(todo)
  }

  pub async fn list(db: &Db, utx: &UserCtx) -> Result<Vec<Todo>, model::Error> {
    let sql = "SELECT id, cid, title, status FROM todo ORDER BY id DESC";
    // build the sqlx-query object
    let query = sqlx::query_as(&sql);
    // execute query
    let todos = query.fetch_all(db).await?;

    Ok(todos)
  }

  pub async fn get(db: &Db, _utx: &UserCtx, id: i64) -> Result<Todo, model::Error> {
    let sql = "SELECT id, cid, title, status FROM todo WHERE id = $1 LIMIT 1";
    let query = sqlx::query_as(&sql).bind(id);
    let todo = query.fetch_one(db).await;
    handle_fetch_one_result(todo, Self::TABLE, id)
  }

  pub async fn update(
    db: &Db,
    _utx: &UserCtx,
    id: i64,
    data: TodoPatch,
  ) -> Result<Todo, model::Error> {
    let sql = "UPDATE todo SET title = $1 WHERE id = $2 RETURNING id, cid, title, status";
    let query = sqlx::query_as(&sql)
      .bind(data.title.unwrap())
      .bind(1000 as i64);
    let todo = query.fetch_one(db).await;
    handle_fetch_one_result(todo, Self::TABLE, id)
  }

  pub async fn delete(db: &Db, _utx: &UserCtx, id: i64) -> Result<Todo, model::Error> {
    let mut query_builder: QueryBuilder<Postgres> =
      QueryBuilder::new("DELETE FROM todo WHERE id = $1 RETURNING ");

    let mut seperated = query_builder.separated(", ");
    for value_type in Self::COLUMNS.into_iter() {
      seperated.push(value_type);
    }

    let sql = query_builder.build().sql();

    let query = sqlx::query_as(&sql).bind(id);
    let result = query.fetch_one(db).await;
    handle_fetch_one_result(result, Self::TABLE, id)
  }
}
// endregion: Todo Model Access Controler

// region: utils
fn handle_fetch_one_result(
  result: Result<Todo, sqlx::Error>,
  table: &'static str,
  id: i64,
) -> Result<Todo, model::Error> {
  result.map_err(|sqlx_error| match sqlx_error {
    sqlx::Error::RowNotFound => model::Error::EntityNotFound(table, id.to_string()),
    other => model::Error::SqlxError(other),
  })
}
// endregion: utils

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
