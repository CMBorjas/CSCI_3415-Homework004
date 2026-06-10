# CSCI 3415 - Homework 4

**Student:** Christian Mandujano Borjas  
**Course:** CSCI 3415

## Live demo

[![Live Demo](https://img.shields.io/badge/Live-Demo-brightgreen.svg)](https://cmborjas.github.io/ResumeWebsite/projects/social-sent-score)

This repository contains the deliverables for Homework 4.

## Tech Stack
- **Rust**: Core application language
- **TypeScript**: Web port implementation
- **Next.js**: Framework for the web showcase
- **Tailwind CSS**: Styling for the web interface
- **Data Parsing**: Core logic for CSV dictionary mapping

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

## Website Integration

This project has been integrated into a web-based portfolio ([ResumeWebsite](https://github.com/CMBorjas/ResumeWebsite)). The original Rust logic was carefully ported to TypeScript, allowing it to run natively within a Next.js application without a server backend.

You can view the fully interactive showcase in the `social-sent-score` page within the portfolio, which features:
- File upload functionality for `.txt` files.
- Live, client-side data parsing from the Stanford SocialSent dataset.
- A cyberpunk-styled interface crafted with Tailwind CSS that displays real-time score accumulation and star ratings.
