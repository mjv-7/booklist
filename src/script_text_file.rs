/*
Made by: Mathew Dusome
April 30 2025
Updated: December 17 2025
To import you need:
Adds TextFile functionality for local file operations

Used for Everything:
   In your main.rs file add this near the top (no mod.rs required):
       #[path = "modules/textfiles.rs"]
       mod textfiles;

   Then add with the other use statements
       use textfiles::TextFile;


Simple script examples:

1. Save different data to separate files:

    // Save string data (player names)
    let names = vec!["Alice", "Bob", "Charlie"];
    let result = TextFile::save_strings("player_names.txt", names);
    if let Err(e) = result {
        println!("Error saving names: {}", e);
    }

    // Save integer data (scores)
    let scores = vec![100, 85, 92];
    let result = TextFile::save_numbers("high_scores.txt", scores);
    if let Err(e) = result {
        println!("Error saving scores: {}", e);
    }

2. Read data from files:

    // Load player names
    let result = TextFile::load_strings("player_names.txt");
    if let Ok(names) = result {
        for name in names {
            println!("Player: {}", name);
        }
    } else if let Err(e) = result {
        println!("Error loading names: {}", e);
    }

    // Load high scores
    let result = TextFile::load_numbers::<i32>("high_scores.txt");
    if let Ok(scores) = result {
        for score in scores {
            println!("Score: {}", score);
        }
    } else if let Err(e) = result {
        println!("Error loading scores: {}", e);
    }
    
Platform notes:
- Local-only: Saves files with the exact filename you provide (include .txt extension)
*/

/// TextFile is a utility module for reading and writing text files
/// that works locally for Rust scripts.
pub struct TextFile;

impl TextFile {
    /// Saves a vector of strings to a local file
    /// Convenience method that takes Vec<&str> directly
    pub fn save_strings<T: AsRef<str>>(name: &str, data: Vec<T>) -> Result<(), String> {
        let string_data: Vec<String> = data.into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();
        let joined = string_data.join("\n");
        // Use the exact filename provided (no auto extension)
        let filename = name;

        std::fs::write(&filename, joined)
            .map_err(|e| format!("Failed to write to file {}: {}", filename, e))
    }
    
    /// Loads a vector of strings from a local file
    #[allow(dead_code)]
    pub fn load_strings(name: &str) -> Result<Vec<String>, String> {
        // Use the exact filename provided (no auto extension)
        let filename = name;

        match std::fs::read_to_string(&filename) {
            Ok(content) => Ok(content.lines().map(|s| s.to_string()).collect()),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Ok(Vec::new()) // Return empty vector if file doesn't exist yet
                } else {
                    Err(format!("Failed to read file {}: {}", filename, e))
                }
            }
        }
    }

    /// Saves a vector of numbers to a local file
    /// Handles any type that can be converted to a string
    #[allow(dead_code)]
    pub fn save_numbers<T: ToString>(name: &str, data: Vec<T>) -> Result<(), String> {
        let string_data: Vec<String> = data.into_iter()
            .map(|n| n.to_string())
            .collect();
        let joined = string_data.join("\n");
        // Use the exact filename provided (no auto extension)
        let filename = name;

        std::fs::write(&filename, joined)
            .map_err(|e| format!("Failed to write to file {}: {}", filename, e))
    }

    /// Loads a vector of numbers from a local file
    /// Handles any type that can be parsed from a string
    #[allow(dead_code)]
    pub fn load_numbers<T>(name: &str) -> Result<Vec<T>, String>
    where
        T: std::str::FromStr,
    {
        let strings = Self::load_strings(name)?;

        let mut numbers = Vec::with_capacity(strings.len());
        for s in strings {
            match s.parse::<T>() {
                Ok(n) => numbers.push(n),
                Err(_) => return Err(format!("Failed to parse '{}' as number", s))
            }
        }

        Ok(numbers)
    }
}