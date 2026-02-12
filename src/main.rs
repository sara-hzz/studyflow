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
        Some(p) => match load_rules(&p) {
            Ok(r) => {
                println!("Loaded rules from: {}", p.display());
                r
            }
            Err(e) => {
                eprintln!("Failed to load rules: {e}");
                return Ok(());
            }
        },
        None => vec![],
    };

    println!("StudyFlow");
    println!("Input : {}", input_dir.display());
    println!("Output: {}", output_dir.display());
    println!("DryRun: {}", dry_run);
    println!();

    let mut moved = 0usize;
    let mut skipped = 0usize;

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

        let course = match_course(&rules, &filename)
            .unwrap_or("Unsorted".to_string());

        let file_type = file_type_folder(&path).to_string();

        let dest_dir = output_dir.join(&course).join(&file_type);
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

    println!("\nDone ✅");
    println!("Moved  : {}", moved);
    println!("Skipped: {}", skipped);

    Ok(())
}

fn print_help() {
    println!("StudyFlow - Study File Organizer");
    println!();
    println!("Usage:");
    println!("  cargo run -- <INPUT_DIR> --rules rules.txt [--dry-run] [--output Study]");
    println!();
    println!("Examples:");
    println!("  cargo run -- ~/Downloads --rules rules.txt --dry-run");
    println!("  cargo run -- ~/Downloads --rules rules.txt");
    println!("  cargo run -- ~/Downloads --rules rules.txt --output Study");
    println!();
    println!("Other:");
    println!("  cargo run -- --version");
}

fn match_course(rules: &[CourseRule], filename: &str) -> Option<String> {
    let name = filename.to_lowercase();
    for rule in rules {
        for kw in &rule.keywords {
            if !kw.is_empty() && name.contains(kw) {
                return Some(rule.course.clone());
            }
        }
    }
    None
}

fn file_type_folder(path: &Path) -> &'static str {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "pdf" => "PDFs",
        "ppt" | "pptx" => "Slides",
        "doc" | "docx" => "Documents",
        "png" | "jpg" | "jpeg" | "gif" | "webp" => "Images",
        "zip" | "rar" | "7z" | "tar" | "gz" => "Archives",
        "pkg" => "Installers",
        "mov" | "mp4" | "mkv" => "Videos",
        "txt" | "md" => "Notes",
        "csv" | "xlsx" => "Data",
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
        let kws = parts.next().unwrap_or("").trim();

        if course.is_empty() || kws.is_empty() {
            continue;
        }

        let keywords: Vec<String> = kws
            .split(',')
            .map(|k| k.trim().to_lowercase())
            .filter(|k| !k.is_empty())
            .collect();

        rules.push(CourseRule {
            course: course.to_string(),
            keywords,
        });
    }

    Ok(rules)
}

fn unique_destination(dest: &Path) -> io::Result<PathBuf> {
    let parent = dest.parent().unwrap_or(Path::new("."));
    let stem = dest.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = dest.extension().and_then(|e| e.to_str());

    for i in 1..=9999 {
        let candidate = match ext {
            Some(ext) => parent.join(format!("{stem}_{i}.{ext}")),
            None => parent.join(format!("{stem}_{i}")),
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
