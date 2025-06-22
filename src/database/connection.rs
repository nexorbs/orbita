use crate::AppWindow;
use crate::models::{ConnectionInfo, DatabaseType};
use slint::Weak;
use sqlx::Row;

pub async fn test_database_connection(
    conn_info: &ConnectionInfo,
    ui_weak: &Weak<AppWindow>,
) -> Result<String, String> {
    let connection_string = conn_info.build_connection_string();
    println!("Probando conexión: {}", connection_string);

    match conn_info.db_type {
        DatabaseType::MySQL => test_mysql_connection(&connection_string, ui_weak).await,
        DatabaseType::PostgreSQL => test_postgresql_connection(&connection_string, ui_weak).await,
        DatabaseType::SQLite => test_sqlite_connection(&connection_string, ui_weak).await,
    }
}

async fn test_mysql_connection(
    connection_string: &str,
    ui_weak: &Weak<AppWindow>,
) -> Result<String, String> {
    match sqlx::MySqlPool::connect(connection_string).await {
        Ok(pool) => {
            match sqlx::query("SELECT VERSION() as version")
                .fetch_one(&pool)
                .await
            {
                Ok(row) => {
                    let version: String = row.get("version");
                    pool.close().await;
                    if let Some(ui) = ui_weak.upgrade() {
                        ui.set_is_db_connected(true);
                    }
                    Ok(format!("✅ Conexión MySQL exitosa - Versión: {}", version))
                }
                Err(e) => Err(format!("❌ Error en consulta MySQL: {}", e)),
            }
        }
        Err(e) => Err(format!("❌ Error conectando a MySQL: {}", e)),
    }
}

async fn test_postgresql_connection(
    connection_string: &str,
    ui_weak: &Weak<AppWindow>,
) -> Result<String, String> {
    match sqlx::PgPool::connect(connection_string).await {
        Ok(pool) => {
            match sqlx::query("SELECT version() as version")
                .fetch_one(&pool)
                .await
            {
                Ok(row) => {
                    let version: String = row.get("version");
                    pool.close().await;
                    if let Some(ui) = ui_weak.upgrade() {
                        ui.set_is_db_connected(true);
                    }
                    Ok(format!("✅ Conexión PostgreSQL exitosa - {}", version))
                }
                Err(e) => Err(format!("❌ Error en consulta PostgreSQL: {}", e)),
            }
        }
        Err(e) => Err(format!("❌ Error conectando a PostgreSQL: {}", e)),
    }
}

async fn test_sqlite_connection(
    connection_string: &str,
    ui_weak: &Weak<AppWindow>,
) -> Result<String, String> {
    match sqlx::SqlitePool::connect(connection_string).await {
        Ok(pool) => {
            match sqlx::query("SELECT sqlite_version() as version")
                .fetch_one(&pool)
                .await
            {
                Ok(row) => {
                    let version: String = row.get("version");
                    pool.close().await;
                    if let Some(ui) = ui_weak.upgrade() {
                        ui.set_is_db_connected(true);
                    }
                    Ok(format!("✅ Conexión SQLite exitosa - Versión: {}", version))
                }
                Err(e) => Err(format!("❌ Error en consulta SQLite: {}", e)),
            }
        }
        Err(e) => Err(format!("❌ Error conectando a SQLite: {}", e)),
    }
}
