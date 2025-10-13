#[tauri::command]
pub fn add_group(
    group_name: String,
    group_subscribe_url: Option<String>,
    group_arguments: Option<String>,
) {
    let message = match (&group_subscribe_url, &group_arguments) {
        (Some(url), Some(args)) => format!("add_group: {} {} {}", group_name, url, args),
        (Some(url), None) => format!("add_group: {} {}", group_name, url),
        (None, Some(args)) => format!("add_group: {} {}", group_name, args),
        (None, None) => format!("add_group: {}", group_name),
    };
    println!("{}", message);
}
