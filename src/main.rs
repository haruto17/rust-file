use std::fmt::write;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn create_file(name: &String) {
    File::create(name).expect("File create failed!");
    println!("File created successfully");
}

fn read_file(name: String) {}

fn write_file(name: &String, text: String) {
    let mut file = OpenOptions::new().append(true).open(name).unwrap();

    file.write_all(text.as_bytes()).unwrap();
}

fn delete_file(name: &String) {
    fs::remove_file(name).expect("File delete failed!");
    println!("File deleted successfully!");
}

fn main() {
    let file_name: String = "Hello.txt".to_string();
    // create_file(&file_name);

    write_file(&file_name, "asdfasdf".to_string());

    // delete_file(&file_name);
}
