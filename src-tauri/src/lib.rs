// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use crate::spary::spary_switch;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri_plugin_sql::{Migration, MigrationKind};

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
        Migration{
            version:3,
            description:"add default group",
            sql:"INSERT INTO `group`(name) VALUES('default')",
            kind:MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:spary.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![spary_switch])
        .setup(|app| {
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_item])?;
            let tray = TrayIconBuilder::new()
                .tooltip("spary")
                .on_menu_event(move |app, event| {
                    if event.id().as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .build(app)?;
            tray.set_menu(Some(menu))?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
