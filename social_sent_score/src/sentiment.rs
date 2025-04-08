// Link to HashMap documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
use std::collections::HashMap;
use std::fs;

pub fn build_social_sentiment_table(file_path: &str) -> HashMap<String, f64> {
    let mut sentiment_table = HashMap::new();

    if let Ok(contents) = fs::read_to_string(file_path) {
        for (i, line) in contents.lines().enumerate() {
            // Skip header
            if i == 0 {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                let word = parts[0].trim().to_lowercase();
                if let Ok(score) = parts[1].trim().parse::<f64>() {
                    sentiment_table.insert(word, score);
                }
            }
        }
    } else {
        eprintln!("Failed to read sentiment file: {}", file_path);
    }

    sentiment_table
}

pub fn get_social_sentiment_score(
    review_file: &str,
    sentiment_table: &HashMap<String, f64>,) 
    -> f64 
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
                total_score += score;
                println!("{}: {:.2}, {:.2}", cleaned, score, total_score);
            }
        }
    } else {
        eprintln!("Failed to read review file: {}", review_file);
    }

    total_score
}


pub fn get_star_rating(score: f64) -> u8 {
    // Determine the star rating based on the score
    match score {
        s if s < -5.0 => 1,
        s if s < -1.0 => 2,
        s if s < 1.0 => 3,
        s if s < 5.0 => 4,
        _ => 5,
    }
}
