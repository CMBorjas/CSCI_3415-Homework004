# CSCI 3415 - Homework 4

**Student:** Christian Mandujano Borjas  
**Course:** CSCI 3415

This repository contains the deliverables for Homework 4.

## Contents

- **`social_sent_score/`**: Contains the Rust implementation of the Social Sentiment Score Analyzer. It includes the source code (`src/`), data files (`data/`), and its own detailed `README.md` with instructions on how to build and run the program.
- **`Homework 4.docx`**: The original homework assignment document containing instructions and requirements.

## Overview

The primary project for this homework is a sentiment analysis tool written in Rust. It reads word sentiment scores from a CSV file (derived from Stanford SocialSent) and analyzes the sentiment of a given review text file. The final output includes the total sentiment score and a corresponding star rating (1–5 stars) based on the accumulated score.

### Usage

To compile and run the application, navigate to the `social_sent_score` directory:

```bash
cd social_sent_score
cargo build --release
cargo run -- data/review.txt
```

For more detailed information, please refer to the [`README.md` inside the `social_sent_score/` directory](./social_sent_score/README.md).

## Website integration 


