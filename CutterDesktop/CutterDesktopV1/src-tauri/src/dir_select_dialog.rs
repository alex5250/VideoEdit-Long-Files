extern crate nfd;

use nfd::Response;
use std::fs;

pub fn folder_select() -> String {
    let result = nfd::open_pick_folder(None).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    match result {
        Response::Okay(file_path) => file_path,
        Response::Cancel => "error".to_string(),
        _ => ("fatal error").to_string(),
    }
}

pub fn mpv_list_perpare(input_path: String) {
    let paths = fs::read_dir(input_path).unwrap();

    for path in paths {
        println!("mpv {}", path.unwrap().path().display())
    }
}
