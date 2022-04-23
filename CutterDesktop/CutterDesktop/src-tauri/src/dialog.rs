use native_dialog::{FileDialog, MessageDialog, MessageType};
extern crate nfd;
use nfd::Response;

pub fn get_file() -> String {
    let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let filename = match result {
        Response::Okay(file_path) => file_path,
        Response::OkayMultiple(files) => "error".to_string(),
        Response::Cancel => "error".to_string(),
    };
    println!("{}", filename);
    return filename;
}
