#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod dir_select_dialog;
use dir_select_dialog::folder_select;
use dir_select_dialog::mpv_list_perpare;

#[tauri::command]
fn directory_select() {
    let result = folder_select();
    match result.as_str() {
        "error" => println!("{}", "error"),
        "fatal error" => println!("{}", "error"),
        _ => mpv_list_perpare(result),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![directory_select])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
