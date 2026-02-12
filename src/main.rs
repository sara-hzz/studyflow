use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct CourseRule {
    course: String,
    keywords: Vec<String>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return Ok(());
    }

    if args.iter().any(|a| a == "--version") {
        println!("StudyFlow v0.1.0");
        return Ok(());
    }

    let input_dir = PathBuf::from(&args[1]);

    let mut rules_path: Option<PathBuf> = None;
    let mut dry_run = false;
    let mut output_dir = PathBuf::from("Study");

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--rules" => {
                if i + 1 >= args.len() {
                    eprintln!("Missing value after --rules");
                    return Ok(());
                }
                rules_path = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--dry-run" => {
                dry_run = true;
                i += 1;
            }
            "--output" => {
                if i + 1 >= args.len() {
                    eprintln!("Missing value after --output");
                    return Ok(());
                }
                output_dir = PathBuf::from(&args[i + 1]);
                i += 2;
            }
            _ => i += 1,
        }
    }

    if !input_dir.exists() || !input_dir.is_dir() {
        eprintln!("Input directory not found: {}", input_dir.display());
        return Ok(());
    }

    let rules: Vec<CourseRule> = match rules_path {
        Some(p) => load_rules(&p)?,
        None => vec![],
    };

    println!("StudyFlow v0.1.0");
    println!("Input : {}", input_dir.display());
    println!("Output: {}", output_dir.display());
    println!("DryRun: {}", dry_run);
    println!();

    let mut moved = 0usize;
    let mut skipped = 0usize;
    let mut unmatched = 0usize;

    for entry in fs::read_dir(&input_dir)? {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => {
                skipped += 1;
                continue;
            }
        };

        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let filename = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => {
                skipped += 1;
                continue;
            }
        };

        if filename.starts_with('.') {
            skipped += 1;
            continue;
        }

        let course = match_course(&rules, &filename);

        if course.is_none() {
            unmatched += 1;
        }

        let course_folder = course.unwrap_or("Unsorted".to_string());
        let file_type = file_type_folder(&path);

        let dest_dir = output_dir.join(&course_folder).join(file_type);
        let mut dest_path = dest_dir.join(&filename);

        if dest_path.exists() {
            dest_path = unique_destination(&dest_path)?;
        }

        if dry_run {
            println!("PLAN  {}  ->  {}", path.display(), dest_path.display());
        } else {
            fs::create_dir_all(&dest_dir)?;
            move_file(&path, &dest_path)?;
            println!("MOVED {}  ->  {}", path.display(), dest_path.display());
        }

        moved += 1;
    }

    println!("\nSummary Report");
    println!("--------------");
    println!("Files processed : {}", moved);
    println!("Unmatched files : {}", unmatched);
    println!("Skipped entries : {}", skipped);

    Ok(())
}

fn print_help() {
    println!("StudyFlow - Rule-based file organizer");
    println!();
    println!("Usage:");
    println!("cargo run -- <INPUT_DIR> --rules rules.txt [--dry-run]");
    println!();
    println!("Options:");
    println!("--rules <file>   Path to rules file");
    println!("--dry-run        Preview without moving files");
    println!("--output <dir>   Custom output folder");
    println!("--version        Show version");
}

fn match_course(rules: &[CourseRule], filename: &str) -> Option<String> {
    let name = filename.to_lowercase();
    for rule in rules {
        for kw in &rule.keywords {
            if name.contains(kw) {
                return Some(rule.course.clone());
            }
        }
    }
    None
}

fn file_type_folder(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase().as_str() {
        "pdf" => "PDFs",
        "ppt" | "pptx" => "Slides",
        "doc" | "docx" => "Documents",
        "png" | "jpg" | "jpeg" => "Images",
        "zip" | "rar" => "Archives",
        "pkg" => "Installers",
        "mov" | "mp4" => "Videos",
        _ => "Other",
    }
}

fn load_rules(path: &Path) -> io::Result<Vec<CourseRule>> {
    let raw = fs::read_to_string(path)?;
    let mut rules = Vec::new();

    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.splitn(2, ':');
        let course = parts.next().unwrap_or("").trim();
        let keywords = parts.next().unwrap_or("").trim();

        if course.is_empty() || keywords.is_empty() {
            continue;
        }

        let keyword_list = keywords
            .split(',')
            .map(|k| k.trim().to_lowercase())
            .collect();

        rules.push(CourseRule {
            course: course.to_string(),
            keywords: keyword_list,
        });
    }

    Ok(rules)
}

fn unique_destination(dest: &Path) -> io::Result<PathBuf> {
    let parent = dest.parent().unwrap();
    let stem = dest.file_stem().unwrap().to_string_lossy();
    let ext = dest.extension().and_then(|e| e.to_str());

    for i in 1..1000 {
        let candidate = match ext {
            Some(e) => parent.join(format!("{}_{}.{}", stem, i, e)),
            None => parent.join(format!("{}_{}", stem, i)),
        };
        if !candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(io::Error::new(io::ErrorKind::Other, "Too many duplicates"))
}

fn move_file(from: &Path, to: &Path) -> io::Result<()> {
    match fs::rename(from, to) {
        Ok(_) => Ok(()),
        Err(_) => {
            fs::copy(from, to)?;
            fs::remove_file(from)?;
            Ok(())
        }
    }
}
