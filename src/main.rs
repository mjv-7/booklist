/*
By: Mujibullah
Date: 2026-04-20
Program Details: A program where you can add/remove or see books.
*/
mod script_text_file;
use script_text_file::TextFile;
use std::io;
//remove check
// add check
//exit check
//view check
fn main() {
    println!(
        "Hi to the library program:
    If you would like to add a book, please type add
    if you would like to see all the books, please view
    if you would like to remove a book, please remove 
    if you would like to search a specific book, please search
    if you would like to exit, please exit"
    );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: please enter a string");
    if input.trim().to_lowercase() == "add" {
        save_info();
    } else if input.trim().to_lowercase() == "view" {
        print_data();
    } else if input.trim().to_lowercase() == "remove" {
        remove_data();
    }else if input.trim().to_lowercase() == "search" {
        search_data();
    } 
    else if input.trim().to_lowercase() == "exit" {
        std::process::exit(0);
    }
}
fn save_info() {
    let mut names = TextFile::load_strings("textfile.txt").unwrap();
    loop {
        println!("Enter the name of the book: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: please enter a string");
        names.push(input.trim().to_lowercase().to_string());
        println!("Would you like to add another book? (yes/no): ");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("error: please enter a string");
        if again.trim().to_lowercase() == "no" {
            break;
        }
    }
    // Save string data (player names)
    let result = TextFile::save_strings("textfile.txt", names);
    if let Err(e) = result {
        println!("Error saving names: {}", e);
    }
}
fn print_data() {
    // Load player names
    let result = TextFile::load_strings("textfile.txt");
    if let Ok(names) = result {
        for name in names {
            println!(": {}", name);
        }
    } else if let Err(e) = result {
        println!("Error loading names: {}", e);
    }
}
fn remove_data() {
    println!("Enter the name of the book you would like to remove: ");
    let mut result = TextFile::load_strings("textfile.txt").unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: please enter a string");
    for i in 0..result.len() {
        if input.trim().to_lowercase() == result[i].to_lowercase() {
            result.remove(i);
            break;
        }
    }

    // Save string data (player names)
    let result = TextFile::save_strings("textfile.txt", result);
    if let Err(e) = result {
        println!("Error saving names: {}", e);
    }
}
fn search_data(){
    println!("Enter the name of the book you would like to search: ");
    let result = TextFile::load_strings("textfile.txt").unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: please enter a string");
    for i in 0..result.len() {
        if input.trim().to_lowercase() == result[i].to_lowercase() {
            println!("Book found: {}", result[i]);
            break;
        }
    }
}