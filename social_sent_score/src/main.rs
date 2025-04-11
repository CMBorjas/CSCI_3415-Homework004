use std::collections::HashMap;
use std::env;

mod sentiment;
use sentiment::{build_social_sentiment_table, get_social_sentiment_score, get_star_rating};

fn main() {
    // Load the sentiment table from the CSV file
    // I create a hashmap from the CSV file
    let sentiment_table: HashMap<String, f64> =
        build_social_sentiment_table("data/socialsent.csv");

    // Get command-line argument or what ever you want...
    let args: Vec<String> = env::args().collect();
    // Get the review file from the command-line argument (default file)
    let review_file = args.get(1).map(String::as_str).unwrap_or("data/review.txt");
    
    // Print the review file being analyzed
    println!("Analyzing file: {}", review_file);

    // Get the sentiment score for the review file using the fucntion...duh
    let total_score = get_social_sentiment_score(review_file, &sentiment_table);

    // Get the star rating
    let stars = get_star_rating(total_score);

    // Print the results
    println!("\n{} score: {:.2}", review_file, total_score);
    // Print the star rating
    println!("{} Stars: {}", review_file, stars);
}
