#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


/*
todo: lazystatic mut instead that I use now.
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("called {}", ARRAY.lock().unwrap().len());
}
*/


mod dir_select_dialog;
mod parse;
mod regrex_check;
use dir_select_dialog::folder_select;
use dir_select_dialog::global_path;
use dir_select_dialog::mpv_list_perpare;
use parse::parse_file;
use parse::CuttedClip;
use std::env;
use crate::regrex_check::get_file_name;
use std::path::Path;
use nfd::Response;
use std::fs;

#[tauri::command]
fn directory_select() {
    let result = folder_select();
    env::set_current_dir(&result).is_ok();
    let output = match result.as_str() {
        "error" => println!("{}", "error"),
        "fatal error" => println!("{}", "error"),
        _ => mpv_list_perpare(result),
    };
}

#[tauri::command]
fn table_render() -> Vec<CuttedClip>  {
    unsafe {
    let path=Path::new("..").join(global_path.clone()).join("config.dat");
    let final_path=match  path.to_str() {
        Some(path) => path,
        None => "none"
    };
    return parse_file(final_path.to_string());
}
}
#[tauri::command]
fn render(input: Vec<CuttedClip>) -> String {
    let mut output=String::new();
    for a in input {
        let filename = get_file_name(a.comment.clone(), "mp4".to_string());
        let ffmpeg = format!(
            "ffmpeg -ss {} -to {}  -i {} -vcodec copy {}\n",
            a.start_time, a.end_time, a.file_belong, filename
        );    
        output.push_str(&ffmpeg);

        
    }
    return output;
    }
    #[tauri::command]
    fn save(input: String)  {
        let result = nfd::open_save_dialog(None, None).unwrap_or_else(|e| {
            panic!("{}",e);
        });

        let result_path=match result {
            Response::Okay(file_path) => file_path,
            Response::Cancel => "error".to_string(),
            Response::OkayMultiple(_) => "error".to_string(),

        };
        fs::write(result_path, input).expect("Unable to write file");
    }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![directory_select,table_render,render,save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
