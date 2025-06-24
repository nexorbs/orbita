use crate::AppWindow;
use crate::database::{DatabasePool, database_connection, execute_query, test_database_connection};
use crate::models::{ConnectionInfo, DatabaseType, QueryResult};
use serde_json::Value;
use serde_json::json;
use slint::{ComponentHandle, SharedString, Weak};
use sqlx::pool;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

pub fn setup_ui_handlers(ui: &AppWindow, rt: Arc<Runtime>) -> Arc<Mutex<Option<DatabasePool>>> {
    let ui_weak = ui.as_weak();
    let database_pool: Arc<Mutex<Option<DatabasePool>>> = Arc::new(Mutex::new(None));

    setup_test_connection_handler(&ui_weak, &database_pool, &rt);
    setupt_connection_handler(&ui_weak, &database_pool, &rt);
    setup_execute_query_handler(&ui_weak, &database_pool, &rt);

    database_pool
}

fn setup_test_connection_handler(
    ui_weak: &Weak<AppWindow>,
    database_pool: &Arc<Mutex<Option<DatabasePool>>>,
    rt: &Arc<Runtime>,
) {
    let ui_weak = ui_weak.clone();
    let database_pool = database_pool.clone();
    let rt = rt.clone();

    ui_weak.upgrade().unwrap().on_rTestConnection(move || {
        let ui = ui_weak.upgrade().unwrap();
        let conn_info = ConnectionInfo::new(
            ui.get_connection_name().to_string(),
            DatabaseType::from_string(&ui.get_database_type()).unwrap_or(DatabaseType::PostgreSQL),
            ui.get_db_host().to_string(),
            ui.get_db_port().to_string(),
            ui.get_db_name().to_string(),
            ui.get_db_user().to_string(),
            ui.get_db_password().to_string(),
        );

        println!("🔄 Probando conexión: {}", conn_info.name);

        let test_result = rt.block_on(test_database_connection(&conn_info, &ui_weak));
        match test_result {
            Ok(message) => {
                println!("{}", message);
                let pool_result = rt.block_on(DatabasePool::new(&conn_info));
                match pool_result {
                    Ok(pool) => {
                        let mut db_pool = database_pool.lock().unwrap();
                        *db_pool = Some(pool);
                        if let Some(ui) = ui_weak.upgrade() {
                            ui.set_ui_db_name(SharedString::from(conn_info.name));
                        }
                        println!("✅ Pool de conexiones creado exitosamente");
                    }
                    Err(error) => {
                        println!("❌ Error creando pool de conexiones: {}", error);
                    }
                }
                if let Some(pool) = database_pool.lock().unwrap().as_ref() {
                    rt.block_on(pool.close());
                    println!("✅ Pool de conexiones cerrada exitosamente");
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    });
}

fn setupt_connection_handler(
    ui_weak: &Weak<AppWindow>,
    database_pool: &Arc<Mutex<Option<DatabasePool>>>,
    rt: &Arc<Runtime>,
) {
    let ui_weak = ui_weak.clone();
    let database_pool = database_pool.clone();
    let rt = rt.clone();

    ui_weak.upgrade().unwrap().on_rAddConnection(move || {
        let ui = ui_weak.upgrade().unwrap();
        let conn_info = ConnectionInfo::new(
            ui.get_connection_name().to_string(),
            DatabaseType::from_string(&ui.get_database_type()).unwrap_or(DatabaseType::PostgreSQL),
            ui.get_db_host().to_string(),
            ui.get_db_port().to_string(),
            ui.get_db_name().to_string(),
            ui.get_db_user().to_string(),
            ui.get_db_password().to_string(),
        );

        println!("🔄 Probando conexión: {}", conn_info.name);

        let test_result = rt.block_on(database_connection(&conn_info, &ui_weak));
        match test_result {
            Ok(message) => {
                println!("{}", message);
                let pool_result = rt.block_on(DatabasePool::new(&conn_info));
                match pool_result {
                    Ok(pool) => {
                        let mut db_pool = database_pool.lock().unwrap();
                        *db_pool = Some(pool);
                        if let Some(ui) = ui_weak.upgrade() {
                            ui.set_ui_db_name(SharedString::from(conn_info.name));
                        }
                        println!("✅ Pool de conexiones creado exitosamente");
                    }
                    Err(error) => {
                        println!("❌ Error creando pool de conexiones: {}", error);
                    }
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    });
}

fn setup_execute_query_handler(
    ui_weak: &Weak<AppWindow>,
    database_pool: &Arc<Mutex<Option<DatabasePool>>>,
    rt: &Arc<Runtime>,
) {
    let ui_weak = ui_weak.clone();
    let database_pool = database_pool.clone();
    let rt = rt.clone();

    ui_weak.upgrade().unwrap().on_rExecuteQuery(move || {
        let ui = ui_weak.upgrade().unwrap();
        let query = ui.get_query_text().to_string();

        let pool_guard = database_pool.lock().unwrap();
        if let Some(ref pool) = *pool_guard {
            let pool_clone = pool.clone();
            drop(pool_guard);

            println!("🔍 Ejecutando query: {}", query);

            let query_result = rt.block_on(execute_query(&pool_clone, &query));
            match query_result {
                Ok(QueryResult::Select(results)) => {
                    println!(
                        "✅ Query SELECT ejecutada exitosamente. {} filas retornadas",
                        results.len()
                    );

                    let json_rows: Vec<Value> = results
                        .iter()
                        .map(|row| {
                            let obj = row
                                .iter()
                                .map(|(k, v)| (k.clone(), v.clone()))
                                .collect::<serde_json::Map<_, _>>();
                            Value::Object(obj)
                        })
                        .collect();

                    println!("{}", serde_json::to_string_pretty(&json_rows).unwrap());

                    // Optionally, set it to the UI if you have a method for that:
                    // if let Some(ui) = ui_weak.upgrade() {
                    //     ui.set_query_results(SharedString::from(serde_json::to_string(&json_rows).unwrap()));
                    // }
                }
                Ok(QueryResult::Modify(rows_affected)) => {
                    println!(
                        "✅ Query de modificación ejecutada exitosamente. {} filas afectadas",
                        rows_affected
                    );
                }
                Err(error) => {
                    println!("❌ Error ejecutando query: {}", error);
                }
            }
        } else {
            println!("❌ No hay conexión activa a la base de datos");
        }
    });
}
