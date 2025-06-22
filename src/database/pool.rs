use crate::models::ConnectionInfo;
use sqlx::{MySql, Pool, Postgres, Sqlite};
use std::error::Error;

#[derive(Debug, Clone)]
pub enum DatabasePool {
    PostgreSQL(Pool<Postgres>),
    MySQL(Pool<MySql>),
    SQLite(Pool<Sqlite>),
}

impl DatabasePool {
    pub async fn new(conn_info: &ConnectionInfo) -> Result<Self, Box<dyn Error>> {
        let connection_string = conn_info.build_connection_string();

        match conn_info.db_type {
            crate::models::DatabaseType::PostgreSQL => {
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(10)
                    .connect(&connection_string)
                    .await?;
                Ok(DatabasePool::PostgreSQL(pool))
            }
            crate::models::DatabaseType::MySQL => {
                let pool = sqlx::mysql::MySqlPoolOptions::new()
                    .max_connections(10)
                    .connect(&connection_string)
                    .await?;
                Ok(DatabasePool::MySQL(pool))
            }
            crate::models::DatabaseType::SQLite => {
                let pool = sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(10)
                    .connect(&connection_string)
                    .await?;
                Ok(DatabasePool::SQLite(pool))
            }
        }
    }

    pub async fn close(&self) {
        match self {
            DatabasePool::PostgreSQL(pool) => pool.close().await,
            DatabasePool::MySQL(pool) => pool.close().await,
            DatabasePool::SQLite(pool) => pool.close().await,
        }
    }
}
