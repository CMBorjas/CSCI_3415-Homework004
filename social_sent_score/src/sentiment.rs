// Link to HashMap documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
use std::collections::HashMap;
use std::fs;
//----------------------------------------------------------------------------
// function uses a file and a pointer to build a hashmap
// the hashmap uses string and floating point numbers 64 bit
pub fn build_social_sentiment_table(file_path: &str) -> HashMap<String, f64> {
    
    // The map is mutable, so we can add and remove stuff
    let mut sentiment_table = HashMap::new();
    
    // Read the CSV file and populate the sentiment table
    if let Ok(contents) = fs::read_to_string(file_path) {
        for (i, line) in contents.lines().enumerate() {
            
            // Skip header cuse we don't need it
            if i == 0 {
                continue;
            }
            // Split the line by comma and trim whitespace
            let parts: Vec<&str> = line.split(',').collect();
            
            // Check if the line has at least two parts (word and score)
            if parts.len() >= 2 {
                
                // Insert the word and score into the sentiment table
                let word = parts[0].trim().to_lowercase();
                
                // Parse the score as floating point 64 bit and insert
                if let Ok(score) = parts[1].trim().parse::<f64>() {
                    
                    // check if empty 
                    sentiment_table.insert(word, score);
                }
            }
        }
    } else {
        // PANIC! at the disco! (print an error message)
        eprintln!("Failed to read sentiment file: {}", file_path);
    }
    // Return the sentiment table (No return statement needed, because convetion)
    sentiment_table
}
//----------------------------------------------------------------------------
// function to get the sentiment score for the reviewing file
pub fn get_social_sentiment_score(
    review_file: &str,
    sentiment_table: &HashMap<String, f64>,) 
    -> f64 
    // Function takes pointer to file and hashmap pointer, then returns a floating point 64 bit number
    {
    
        // Calculate the sentiment score for the review file
    let mut total_score = 0.0;
    
    // Print the header for the output
    println!("[word: current_score, accumulated_score]");
    
    // Read the review file and calculate the score
    if let Ok(contents) = fs::read_to_string(review_file) {
    
        // Split the contents into words and process each word
        for word in contents.split_whitespace() {
    
            // Clean the word by removing non-alphabetic characters and converting to lowercase
            let cleaned = word.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string();
    
            // Check if the cleaned word exists in the sentiment table
            if let Some(score) = sentiment_table.get(&cleaned) {
                // Binary operator 
                total_score += score;

                // Print the word and its score
                println!("{}: {:.2}, {:.2}", cleaned, score, total_score);
            }
        }
    } else {
        // Two buttons you never press, panic and snooze (print an error message)
        eprintln!("Failed to read review file: {}", review_file);
    }
    // Return the total score (rust convention)
    total_score
}
//----------------------------------------------------------------------------
pub fn get_star_rating(score: f64) -> u8 {
    // Determine the star rating based on the score
    // No return statement given
    // will return a number between 1 and 5 as it is the last line of the function
    // It will be an unsigned 8 bit integer (u8)
    match score {
        s if s < -5.0 => 1,
        s if s < -1.0 => 2,
        s if s < 1.0 => 3,
        s if s < 5.0 => 4,
        _ => 5,
    }
}
