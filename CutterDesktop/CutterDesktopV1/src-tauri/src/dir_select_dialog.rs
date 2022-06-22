/* todo
replace println!() with logger in app
*/

extern crate nfd;

use nfd::Response;
use std::fs;
use std::process::Command;

pub static mut global_path: String = String::new();

pub fn folder_select() -> String {
    let result = nfd::open_pick_folder(None).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let output = match result {
        Response::Okay(file_path) => file_path,
        Response::Cancel => "error".to_string(),
        _ => ("fatal error").to_string(),
    };
   

    unsafe {
        global_path = output.clone();
    }
    return output;
}

fn run_command(path: String, extra_path: String) -> Vec<u8> {
    let mut mpv = Command::new("mpv");
    mpv.arg(path).arg(extra_path);
    let result = mpv.output();
    match result {
        Ok(result) => result.stdout,
        Err(result) => String::from("error").as_bytes().to_vec(),
    }
}
pub fn mpv_list_perpare(input_path: String) {
    let paths = fs::read_dir(input_path).unwrap();
    for path in paths {
        let input_path_as_argument = path.unwrap().path().display().to_string();
        let output = run_command(input_path_as_argument, "".to_string());
        println!("{:#?}", std::str::from_utf8(&output));
    }
}
