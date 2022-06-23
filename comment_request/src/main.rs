use dialog::DialogBox;

fn main() {


    let name = dialog::Input::new("Please enter your comment for video.")
        .title("Comment requred.")
        .show()
        .expect("Could not display dialog box");
    match name {
        Some(name) => println!("{}",name),
        None => println!("no comment provided"),
    };
}
