#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod dialog;
mod parse;
mod regrex_check;
use crate::parse::CuttedClip;
use crate::regrex_check::get_file_name;
use dialog::get_file;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use parse::parse_file;
use std::thread;
use std::io;
use std::fs::File;
use std::io::prelude::*;

fn write_script(ffmpeg:String) -> io::Result<()> {
    println!("{}",ffmpeg);
    let mut file = File::create("./output/build.cmd")?;
    // Early return on error
    file.write_all(ffmpeg.as_bytes())?;
    Ok(())
}

#[tauri::command]
fn render(input: Vec<CuttedClip>) {
    


    let mut output=String::new();

   
    for a in input {
        let filename = get_file_name(a.comment.clone(), "mp4".to_string());
        let ffmpeg = format!(
            "ffmpeg -ss {} -to {}  -i {} -vcodec copy {}\n",
            a.end_time, a.start_time, a.file_belong, filename
        );
      
        output.push_str(&ffmpeg);

        
    }
    write_script(output);
    }

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
        let clip = CuttedClip {
            id: (-1).to_string(),
            start_time: String::new(),
            end_time: String::new(),
            comment: String::new(),
            file_belong: String::new(),
        };
        cutted_clips.push(clip);
        cutted_clips
    } else {
        parse_file(path.to_string())
    }
}

fn main() {
    tauri::Builder::default()
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![select_file_dialog, render])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
