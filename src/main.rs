/*
By: Mujibullah
Date: 2026-04-20
Program Details: A program where you can add/remove or see books.
*/
mod script_text_file;
use script_text_file::TextFile;
use std::io;


fn main() {
    save_info();
    print_data();
}
fn save_info() {
    loop {
        println!("Enter the name of the book: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: please enter a string");

        println!("Would you like to add another book? (yes/no): ");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("error: please enter a string");
        if again.trim().to_lowercase() == "no" {
            break;
        }

        // Save string data (player names)
        let names = vec![input.trim()];
        let result = TextFile::save_strings("textfile.txt", names);
        if let Err(e) = result {
            println!("Error saving names: {}", e);
        }
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
