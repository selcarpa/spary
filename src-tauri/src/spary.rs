
#[tauri::command]
pub fn spary_switch(status:bool) {
    println!("Spraying {status}");
}
