

## Author

CSCI 3415 - Homework 4  
Student: Christian Mandujano Borjas  
Language: Rust  
Date: [April 2025]

# Homework 4 - Social Sentiment Score Analyzer (Rust)

## Overview

This project implements a small sentiment analysis tool in Rust that reads word sentiment scores from a CSV file and analyzes the sentiment of a given review file. The final output includes the total sentiment score and a star rating (1–5 stars) based on the accumulated score.

---

## Objectives

This project is designed to help practice:

- File I/O and data parsing
- Use of dictionaries (Rust `HashMap`)
- String manipulation
- Multi-way selection (e.g., `match` in Rust)
- Command-line argument parsing
- Writing modular, idiomatic Rust code

---

## Implementation Language

**Language Used:** Rust
Rust was selected for its unique ownership model, memory safety, and strong type system.  
All code was written and tested using Rust 1.76+ via `cargo`.

---

## Program Structure

### Subprograms Used

1. **`build_social_sentiment_table(file_path: &str) -> HashMap<String, f64>`**  
   Loads the CSV file into a dictionary of word → sentiment score.

2. **`get_social_sentiment_score(review_file: &str, sentiment_table: &HashMap<String, f64>) -> f64`**  
   Calculates the total sentiment score by summing all known word scores from the review.

3. **`get_star_rating(score: f64) -> u8`**  
   Converts the final score into a star rating (1 to 5 stars) using a multi-way selection strategy (`match`).

---

## Input Files

Place the following files in the root directory:

- `socialsent.csv` – Word → sentiment score CSV (downloaded from Stanford SocialSent)
- `review.txt` – Default review file (optional)
- `good.txt`, `bad.txt` – Sample test files (optional)

Each review file should contain lowercase, space-separated words.  
Each line in `socialsent.csv` should follow the format:
```
word,score
```

---

## Usage

Compile the program using Cargo:
```bash
cargo build --release
```

Run the program:
```bash
cargo run -- <review_file>
```

If no argument is passed, the program defaults to `review.txt`.

### Example Output
```bash
$ cargo run -- short.txt
[word: current_score, accumulated_score]
takes: 0.03, 0.03
multiple: -0.35, -0.32
use: 0.1, -0.22
...
short.txt score: 0.47
short.txt Stars: 3
```

---

## Deliverables

REPL and local folder includes:

- All source code files (`main.rs`, helper modules)
- `socialsent.csv`, test input files
- Output examples (`output.txt`)
- This `README.md`

---

## Notes

- This project was completed with assistance from AI tools like ChatGPT.
- No libraries outside of Rust's standard library were used.
- The program prioritizes correctness, memory safety, and idiomatic Rust style.
- Execution has been tested successfully on Repl.it and locally using Cargo.

---

## Homework 4 Report

### Abstract
Rust is a systems programming language focused on safety, concurrency, and performance. Its unique feature is the ownership model, which guarantees memory safety without needing a garbage collector. This project leverages Rust to build a reliable command-line tool for sentiment analysis.

### Approach
1. **CSV Parsing**: The application reads the `socialsent.csv` dataset line-by-line using `std::fs` and populates a `HashMap` mapping words to their floating-point sentiment scores.
2. **Scoring**: It reads the input review text file, splits it by whitespace, cleans punctuation, and matches words against the `HashMap` in `O(1)` time. It maintains a running total of the score.
3. **Star Rating**: A multi-way selection using Rust's `match` construct assigns a 1 to 5-star rating based on the accumulated score thresholds.

### New Things Learned
- **Ownership & Borrowing**: Passing variables as references (e.g., `&HashMap`) to prevent the function from taking ownership and moving the variable out of scope.
- **HashMap**: Initializing, mutating, and retrieving items using the `std::collections::HashMap` standard library.
- **CLI Handling**: Capturing command-line arguments dynamically using `std::env::args`.

### Likes/Dislikes of Rust
**Likes:**
- The compiler acts like a strict but helpful tutor. Error messages are incredibly detailed and often suggest the exact fix.
- Pattern matching (`match`) is elegant and safer than standard `switch` statements.

**Dislikes:**
- The steep learning curve associated with lifetimes and the borrow checker. Strings in Rust (`String` vs `&str`) are significantly more complex than in Python or Java.
