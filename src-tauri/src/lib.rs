use crate::spary::spary_switch;
use std::sync::Arc;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::Manager;
use tauri_plugin_sql::{Migration, MigrationKind};
use tokio::sync::Mutex;

mod mcp_server;
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
                //Linux: Unsupported. The event is not emitted even though the icon is shown and will still show a context menu on right click.
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                        println!("unhandled event {event:?}");
                    }
                })
                .build(app)?;
            tray.set_menu(Some(menu))?;

            // Start the MCP HTTP server in a background task
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                start_mcp_server(app_handle);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn start_mcp_server(app_handle: tauri::AppHandle) {
    // Disable MCP server for now
    if true {
        return;
    } else {
        use mcp_server::AppState;

        // Create the shared application state
        let state = AppState {
            app_handle,
            spray_status: Arc::new(Mutex::new(false)), // Initial spray status is off
        };
        let port = 17564;
        let host = "127.0.0.1";

        // Configure Rocket with custom settings
        let rocket = rocket::build()
            .configure(rocket::Config {
                port,
                address: host.parse().expect("Invalid address"),
                ..rocket::Config::default()
            })
            .manage(state)
            .mount(
                "/",
                rocket::routes![
                    mcp_server::initialize,
                    mcp_server::get_capabilities,
                    mcp_server::list_tools,
                    mcp_server::call_tool,
                    mcp_server::set_spray_status,
                    mcp_server::get_spray_status
                ],
            );

        println!("MCP server running on http://{host}:{port}");

        // Start the server
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                rocket.launch().await.unwrap();
            });
    }
}
