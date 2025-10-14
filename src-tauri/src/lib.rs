// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use crate::group::add_group;
use crate::spary::spary_switch;
use tauri_plugin_sql::{Migration, MigrationKind};
mod group;
mod spary;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE `group`(id INTEGER PRIMARY KEY AUTOINCREMENT, name VARCHAR(60) NOT NULL,url TEXT NULL, arguments JSON NOT NULL default '{}', created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP)",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "add_node_table",
            sql: "CREATE TABLE node(id INTEGER PRIMARY KEY AUTOINCREMENT, alias VARCHAR(60) NOT NULL,arguments JSON NOT NULL default '{}', created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP, group_id INTEGER NOT NULL)",
            kind: MigrationKind::Up,
        },
    ];
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:spary.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![spary_switch, add_group])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
