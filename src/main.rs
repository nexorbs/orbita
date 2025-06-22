#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::sync::Arc;
use tokio::runtime::Runtime;

slint::include_modules!();

mod database;
mod models;
mod ui;

use ui::setup_ui_handlers;

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Arc::new(Runtime::new()?);
    let ui = AppWindow::new()?;

    let _database_pool = setup_ui_handlers(&ui, rt);

    ui.run()?;
    Ok(())
}
