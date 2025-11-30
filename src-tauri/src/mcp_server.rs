use rocket::{State, serde::{Deserialize, Serialize, json::Json}};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::AppHandle;

use crate::spary::spary_switch;

#[derive(Clone)]
pub struct AppState {
    pub app_handle: AppHandle,
    pub spray_status: Arc<Mutex<bool>>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InitializeResponse {
    protocol_version: String,
    server_info: ServerInfo,
    capabilities: Capabilities,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ServerInfo {
    name: String,
    version: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Capabilities {
    experimental: Option<Value>,
    tools: Option<Value>,
}

#[rocket::post("/mcp/initialize", data = "<_data>")]
pub async fn initialize(_state: &State<AppState>, _data: Json<Value>) -> Json<InitializeResponse> {
    Json(InitializeResponse {
        protocol_version: "1.0".to_string(),
        server_info: ServerInfo {
            name: "spary-mcp-server".to_string(),
            version: "0.1.0".to_string(),
        },
        capabilities: Capabilities {
            experimental: None,
            tools: Some(json!({
                "list": true,
                "call": true
            })),
        },
    })
}

#[rocket::get("/mcp/capabilities")]
pub async fn get_capabilities(_state: &State<AppState>) -> Json<Value> {
    Json(json!({
        "tools": {
            "list": true,
            "call": true
        }
    }))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Tool {
    name: String,
    description: String,
    input_schema: Value,
}

#[rocket::get("/mcp/tools/list")]
pub async fn list_tools(_state: &State<AppState>) -> Json<Value> {
    let tools = vec![
        Tool {
            name: "spray_toggle".to_string(),
            description: "Toggle the spray functionality on or off".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "status": {
                        "type": "boolean",
                        "description": "Whether to turn spray on (true) or off (false)"
                    }
                },
                "required": ["status"]
            }),
        }
    ];

    Json(json!({ "tools": tools }))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ToolCallRequest {
    name: String,
    arguments: Value,
}

#[rocket::post("/mcp/tools/call", data = "<payload>")]
pub async fn call_tool(
    state: &State<AppState>,
    payload: Json<ToolCallRequest>,
) -> Result<Json<Value>, rocket::http::Status> {
    match payload.name.as_str() {
        "spray_toggle" => {
            if let Some(status) = payload.arguments.get("status").and_then(|v| v.as_bool()) {
                // Call the existing spray_switch function
                spary_switch(status);

                // Update the spray status in the shared state
                {
                    let mut spray_status = state.spray_status.lock().await;
                    *spray_status = status;
                }

                Ok(Json(json!({
                    "result": {
                        "success": true,
                        "message": format!("Spray toggled to {}", status)
                    }
                })))
            } else {
                Err(rocket::http::Status::BadRequest)
            }
        }
        _ => Err(rocket::http::Status::NotFound),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SprayRequest {
    status: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SprayResponse {
    success: bool,
    message: String,
}

#[rocket::post("/v1/spray", data = "<payload>")]
pub async fn set_spray_status(
    state: &State<AppState>,
    payload: Json<SprayRequest>,
) -> Json<SprayResponse> {
    // Call the existing spray_switch function
    spary_switch(payload.status);

    // Update the spray status in the shared state
    {
        let mut spray_status = state.spray_status.lock().await;
        *spray_status = payload.status;
    }

    Json(SprayResponse {
        success: true,
        message: format!("Spray toggled to {}", payload.status),
    })
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SprayStatusResponse {
    status: bool,
}

#[rocket::get("/v1/spray")]
pub async fn get_spray_status(
    state: &State<AppState>,
) -> Json<SprayStatusResponse> {
    let spray_status = state.spray_status.lock().await;
    Json(SprayStatusResponse {
        status: *spray_status,
    })
}

