StudyFlow

A lightweight rule-based CLI tool written in Rust to automatically organize academic files into a structured directory system.



Overview

StudyFlow scans a selected folder (e.g. Downloads), applies custom keyword rules, detects file types, and builds a clean study-oriented structure automatically.

It was developed as a practical automation tool and as a structured Rust learning project focused on file systems and command-line applications.



Core Features

Keyword-based course matching

Automatic file type categorization (PDFs, Images, Archives, etc.)

Fallback Unsorted folder

Duplicate-safe file moving

--dry-run preview mode

Clean Git structure with proper .gitignore



How It Works

Rules are defined inside rules.txt using the format:

Course_Name: keyword1, keyword2



The program:

Reads the rules file

Scans the input directory

Matches filenames against keywords

Creates the following structure:

Study/<Course>/<FileType>/


If no rule matches, files are placed in:

Study/Unsorted/



Demo

<p align="center">
  <img src="assets/before.png" width="45%" />
  <img src="assets/after.png" width="45%" />
</p>



Usage

Build the project:

cargo build



Preview changes safely:

cargo run -- ~/Downloads --rules rules.txt --dry-run



Organize files:

cargo run -- ~/Downloads --rules rules.txt

Project Structure
studyflow/
 ├── src/
 ├── rules.txt
 ├── assets/
 ├── README.md
 └── Cargo.toml



Technical Notes

Written in Rust

Uses std::fs, PathBuf, and io::Result

Structured CLI argument handling

Designed as a modular automation tool



Author

Sara Hany Zarea
Medical Information Management Student
Interested in digital systems and workflow automation