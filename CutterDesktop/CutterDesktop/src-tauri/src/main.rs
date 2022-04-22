#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod dialog;
mod parse;
use dialog::get_file;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::thread;
use parse::parse_file;
use crate::parse::CuttedClip;
#[tauri::command]
fn select_file_dialog() -> Vec<CuttedClip> {

        let path = r"C:\Users\WindowsSucks\Desktop\config.dat";
        if path == "error" {
            let yes = MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Error happend!")
                .set_text("Error ! Did you selected file &")
                .show_confirm()
                .unwrap();


        let mut cutted_clips = Vec::new();
        let clip=CuttedClip {
                id:(-1).to_string(),
                start_time:String::new(),
                end_time:String::new(),
                comment:String::new(),
                file_belong:String::new(),


             };
        cutted_clips.push(clip);
        cutted_clips

        }
        else {
            parse_file(path.to_string())
        }
   
}

fn main() {
    tauri::Builder::default()
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![select_file_dialog])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
