// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use sqlx::{Column, Row};
use std::collections::HashMap;
use std::error::Error;
use tokio::runtime::Runtime;

slint::include_modules!();

#[derive(Debug, Clone, Serialize, Deserialize)]
enum DatabaseType {
    MySQL,
    PostgreSQL,
    SQLite,
}

impl DatabaseType {
    fn from_string(s: &str) -> Option<Self> {
        match s {
            "MySQL" => Some(DatabaseType::MySQL),
            "PostgreSQL" => Some(DatabaseType::PostgreSQL),
            "SQLite" => Some(DatabaseType::SQLite),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConnectionInfo {
    name: String,
    db_type: DatabaseType,
    host: String,
    port: String,
    database: String,
    user: String,
    password: String,
}

impl ConnectionInfo {
    fn build_connection_string(&self) -> String {
        match self.db_type {
            DatabaseType::MySQL => {
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.user, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::PostgreSQL => {
                format!(
                    "postgresql://{}:{}@{}:{}/{}",
                    self.user, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::SQLite => {
                format!("sqlite:{}", self.database)
            }
        }
    }

    fn get_default_port(&self) -> &str {
        match self.db_type {
            DatabaseType::MySQL => "3306",
            DatabaseType::PostgreSQL => "5432",
            DatabaseType::SQLite => "", // SQLite no usa puerto
        }
    }
}

async fn test_database_connection(conn_info: &ConnectionInfo) -> Result<String, String> {
    let connection_string = conn_info.build_connection_string();
    println!("Probando conexión: {}", connection_string);

    match conn_info.db_type {
        DatabaseType::MySQL => match sqlx::MySqlPool::connect(&connection_string).await {
            Ok(pool) => {
                match sqlx::query("SELECT VERSION() as version")
                    .fetch_one(&pool)
                    .await
                {
                    Ok(row) => {
                        let version: String = row.get("version");
                        pool.close().await;
                        Ok(format!("✅ Conexión MySQL exitosa - Versión: {}", version))
                    }
                    Err(e) => Err(format!("❌ Error en consulta MySQL: {}", e)),
                }
            }
            Err(e) => Err(format!("❌ Error conectando a MySQL: {}", e)),
        },
        DatabaseType::PostgreSQL => match sqlx::PgPool::connect(&connection_string).await {
            Ok(pool) => {
                match sqlx::query("SELECT version() as version")
                    .fetch_one(&pool)
                    .await
                {
                    Ok(row) => {
                        let version: String = row.get("version");
                        pool.close().await;
                        Ok(format!("✅ Conexión PostgreSQL exitosa - {}", version))
                    }
                    Err(e) => Err(format!("❌ Error en consulta PostgreSQL: {}", e)),
                }
            }
            Err(e) => Err(format!("❌ Error conectando a PostgreSQL: {}", e)),
        },
        DatabaseType::SQLite => match sqlx::SqlitePool::connect(&connection_string).await {
            Ok(pool) => {
                match sqlx::query("SELECT sqlite_version() as version")
                    .fetch_one(&pool)
                    .await
                {
                    Ok(row) => {
                        let version: String = row.get("version");
                        pool.close().await;
                        Ok(format!("✅ Conexión SQLite exitosa - Versión: {}", version))
                    }
                    Err(e) => Err(format!("❌ Error en consulta SQLite: {}", e)),
                }
            }
            Err(e) => Err(format!("❌ Error conectando a SQLite: {}", e)),
        },
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;

    let ui = AppWindow::new()?;

    let ui_weak = ui.as_weak();

    ui.on_rTestConnection(move || {
        let ui = ui_weak.upgrade().unwrap();

        let conn_info = ConnectionInfo {
            name: ui.get_connection_name().to_string(),
            db_type: DatabaseType::from_string(&&ui.get_database_type())
                .unwrap_or(DatabaseType::PostgreSQL),
            host: ui.get_db_host().to_string(),
            port: ui.get_db_port().to_string(),
            database: ui.get_db_name().to_string(),
            user: ui.get_db_user().to_string(),
            password: ui.get_db_password().to_string(),
        };

        println!("🔄 Probando conexión: {}", conn_info.name);

        // Ejecutar la prueba de conexión en el runtime de Tokio
        let result = rt.block_on(test_database_connection(&conn_info));

        match result {
            Ok(message) => {
                println!("{}", message);
                // Actualizar la UI para mostrar éxito
            }
            Err(error) => {
                println!("{}", error);
                // Actualizar la UI para mostrar error
            }
        }
    });

    println!("🚀 Iniciando Orbita - Database Manager");
    println!("📊 Listo para conectar con bases de datos MySQL, PostgreSQL y SQLite");

    ui.run()?;

    Ok(())
}
