use crate::database::DatabasePool;
use crate::models::QueryResult;
use serde_json::Value;
use sqlx::{Column, Row};
use std::error::Error;

pub fn is_select_query(query: &str) -> bool {
    let trimmed = query.trim().to_lowercase();
    trimmed.starts_with("select")
        || trimmed.starts_with("with")
        || trimmed.starts_with("show")
        || trimmed.starts_with("describe")
        || trimmed.starts_with("explain")
}

pub async fn execute_query(
    pool: &DatabasePool,
    query: &str,
) -> Result<QueryResult, Box<dyn Error>> {
    if is_select_query(query) {
        let results = execute_select_query(pool, query).await?;
        Ok(QueryResult::Select(results))
    } else {
        let rows_affected = execute_modify_query(pool, query).await?;
        Ok(QueryResult::Modify(rows_affected))
    }
}

async fn execute_select_query(
    pool: &DatabasePool,
    query: &str,
) -> Result<Vec<serde_json::Map<String, Value>>, Box<dyn Error>> {
    match pool {
        DatabasePool::PostgreSQL(pool) => execute_select_postgres(pool, query).await,
        DatabasePool::MySQL(pool) => execute_select_mysql(pool, query).await,
        DatabasePool::SQLite(pool) => execute_select_sqlite(pool, query).await,
    }
}

async fn execute_modify_query(pool: &DatabasePool, query: &str) -> Result<u64, Box<dyn Error>> {
    match pool {
        DatabasePool::PostgreSQL(pool) => {
            let result = sqlx::query(query).execute(pool).await?;
            Ok(result.rows_affected())
        }
        DatabasePool::MySQL(pool) => {
            let result = sqlx::query(query).execute(pool).await?;
            Ok(result.rows_affected())
        }
        DatabasePool::SQLite(pool) => {
            let result = sqlx::query(query).execute(pool).await?;
            Ok(result.rows_affected())
        }
    }
}

async fn execute_select_postgres(
    pool: &sqlx::Pool<sqlx::Postgres>,
    query: &str,
) -> Result<Vec<serde_json::Map<String, Value>>, Box<dyn Error>> {
    let rows = sqlx::query(query).fetch_all(pool).await?;
    convert_rows_to_json(rows)
}

async fn execute_select_mysql(
    pool: &sqlx::Pool<sqlx::MySql>,
    query: &str,
) -> Result<Vec<serde_json::Map<String, Value>>, Box<dyn Error>> {
    let rows = sqlx::query(query).fetch_all(pool).await?;
    convert_rows_to_json(rows)
}

async fn execute_select_sqlite(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    query: &str,
) -> Result<Vec<serde_json::Map<String, Value>>, Box<dyn Error>> {
    let rows = sqlx::query(query).fetch_all(pool).await?;
    convert_rows_to_json(rows)
}

fn convert_rows_to_json<R: Row>(
    rows: Vec<R>,
) -> Result<Vec<serde_json::Map<String, Value>>, Box<dyn Error>>
where
    for<'r> &'r str: sqlx::ColumnIndex<R>,
    for<'r> String: sqlx::Decode<'r, R::Database> + sqlx::Type<R::Database>,
    for<'r> Option<String>: sqlx::Decode<'r, R::Database>,
{
    let mut results = Vec::new();

    for row in rows {
        let mut map = serde_json::Map::new();
        for column in row.columns() {
            let column_name = column.name();
            let value: Option<String> = row.try_get(column_name).ok();
            map.insert(
                column_name.to_string(),
                value.map(Value::String).unwrap_or(Value::Null),
            );
        }
        results.push(map);
    }
    Ok(results)
}
