mod script_text_file;
use script_text_file::TextFile;
/*
By: Mujibullah
Date: 2026-04-20
Program Details: A program where you can add/remove or see books.
*/
fn save_info(){
    // Save string data (player names)
    let names = vec![""];
    let result = TextFile::save_strings("textfile.txt", names);
    input("please enter the name of the books: ");
    if let Err(e) = result {
        println!("Error saving names: {}", e);
    }
}
fn print_data(){
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
fn main() {
    save_info();
    print_data();
    println!("Hello booklist!");
}