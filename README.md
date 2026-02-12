StudyFlow

A rule-based command-line tool written in Rust that automatically organizes study files into a structured folder system.

StudyFlow scans a directory (e.g. Downloads), matches filenames against custom rules, detects file types, and creates a clean academic structure in seconds.

Why StudyFlow?

As a student, my Downloads folder constantly fills up with:

Lecture slides

Exam questions

Certificates

CV documents

Random images and archives

Instead of manually organizing everything, I built a small automation tool to handle it for me.

This project was built to practice:

File system manipulation in Rust

Command-line argument parsing

Rule-based logic

Error handling

Structured program design

How It Works

The program scans a given directory.

It reads matching rules from rules.txt.

It categorizes files by:

Course name (based on keywords)

File type (PDFs, Images, Archives, etc.)

It moves files into:

Study/<Course>/<FileType>/


If no rule matches, files are placed in:

Study/Unsorted/

Demo
Before

Files mixed inside Downloads:

After

Automatically structured output:

Usage

Build the project:

cargo build


Run in preview mode (safe test):

cargo run -- ~/Downloads --rules rules.txt --dry-run


Run and organize files:

cargo run -- ~/Downloads --rules rules.txt

Example Output Structure
Study/
 ├── Foundations_of_Science/
 │    └── PDFs/
 ├── Health_Informatics/
 │    └── PDFs/
 ├── CV_Job/
 │    └── PDFs/
 ├── Certificates_Admin/
 │    └── PDFs/
 └── Unsorted/
      ├── Images/
      ├── Archives/
      └── Other/

Technical Details

Language: Rust

Uses: std::fs, std::env, PathBuf

Handles duplicate filenames

Supports dry-run mode

Ignores hidden/system files

Clean Git structure (.gitignore configured properly)

Project Status

This was my first fully independent Rust CLI project.
The focus was on building a practical automation tool and structuring it cleanly as a portfolio-ready repository.

Future improvements may include:

Regex-based matching

Configurable output directory

JSON rule configuration

Release binaries

Author

Sara Hany Zarea
Bachelor’s Student – Medical Information Management
Interested in digital systems, automation, and structured data workflows.