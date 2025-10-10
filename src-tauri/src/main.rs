// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;

mod entity;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();


fn main() {
    spary_lib::run()
}
