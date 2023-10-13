use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn create_file(name: &String) {
    File::create(name).expect("File create failed!");
    println!("File created successfully");
}

fn read_text(name: &String) {
    let mut file = File::open(name).expect("File not found!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("{}", contents);
}

fn write_text(name: &String, text: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(name)
        .expect("File not found!");

    file.write_all(text.as_bytes()).expect("File write failed!");
}

fn rename_file(from: &String, to: String) {
    fs::rename(from, to).expect("File rename failed!");
}

fn delete_file(name: &String) {
    fs::remove_file(name).expect("File delete failed!");
    println!("File deleted successfully!");
}

fn main() {
    let file_name: String = "Hello.txt".to_string();
    create_file(&file_name);

    write_text(&file_name, "Hello, World!!".to_string());

    read_text(&file_name);

    rename_file(&file_name, "HW.txt".to_string());

    // delete_file(&file_name);
}
