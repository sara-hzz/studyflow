📦 StudyFlow

A simple CLI tool written in Rust to automatically organize study files into structured folders based on custom rules.

This was my first fully independent Rust project, built to practice file system handling, command-line parsing, and structured logic.



✨ What It Does

StudyFlow scans a folder (e.g. Downloads) and:

Detects file type (PDF, images, archives, installers…)

Matches filenames against custom rules

Creates a structured folder hierarchy

Moves files accordingly

Supports a safe --dry-run mode



🧠 Why I Built This

As a student, I constantly download lecture slides, exam files, and notes.
This project started as a practical solution to organize academic files — and became a hands-on way to deepen my understanding of Rust and CLI tools.



⚙️ How It Works

1. Rules are defined inside rules.txt

2. The program scans the input directory

3. Files are categorized by:

Course name (from rules)

File type (PDFs, Images, Archives, etc.)

4. Files are moved into:

Study/<Course>/<Type>/



🚀 Usage

Build the project:

cargo build


Run normally:

cargo run -- ~/Downloads --rules rules.txt


Run in preview mode (safe test):

cargo run -- ~/Downloads --rules rules.txt --dry-run



📂 Example Output Structure
Study/
 ├── Foundations_of_Science/
 │    ├── PDFs/
 │    └── Archives/
 ├── Health_Informatics/
 │    └── PDFs/
 ├── CV_Job/
 │    └── PDFs/
 └── Unsorted/
      ├── Images/
      ├── Installers/
      └── Archives/



🛠️ Tech Stack

Rust

std::fs

Command-line argument parsing

File system manipulation

Error handling with io::Result



📌 Notes

The Study/ folder is ignored in Git (.gitignore)

Only source code is tracked

Designed as a portfolio-ready CLI project



👩‍💻 Author

Sara Hany Zarea
Bachelor’s Student – Medical Information Management