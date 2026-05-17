use clap::Parser;
use colored::*;
use ignore::WalkBuilder;
use rayon::prelude::*;
use regex::Regex;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A high-performance, colored file searcher and replacer in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to search for (regex supported)
    pattern: String,

    /// The replacement string (optional)
    #[arg(short, long)]
    replace: Option<String>,

    /// The directory to search in
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Case-insensitive search
    #[arg(short, long)]
    ignore_case: bool,

    /// Show line numbers
    #[arg(short, long, default_value_t = true)]
    line_numbers: bool,
}

fn main() {
    let args = Args::parse();
    
    // Build regex with options
    let mut regex_builder = regex::RegexBuilder::new(&args.pattern);
    regex_builder.case_insensitive(args.ignore_case);
    
    let re = Arc::new(regex_builder.build().expect("Invalid regex pattern"));

    if let Some(replacement) = &args.replace {
        println!(
            "{} '{}' with '{}' in {}...",
            "Replacing".red().bold(),
            args.pattern.yellow(),
            replacement.green(),
            args.path.display().to_string().magenta()
        );
    } else {
        println!(
            "{} '{}' in {}...",
            "Searching for".cyan(),
            args.pattern.yellow().bold(),
            args.path.display().to_string().magenta()
        );
    }

    // Collect all files respecting .gitignore
    let files: Vec<PathBuf> = WalkBuilder::new(&args.path)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.into_path())
        .collect();

    // Process files in parallel
    files.into_par_iter().for_each(|path| {
        if let Err(e) = process_file(&path, &re, &args) {
            eprintln!("{}: {}", "Error processing".red(), path.display());
            eprintln!("  {}", e);
        }
    });
}

fn process_file(path: &Path, re: &Regex, args: &Args) -> std::io::Result<()> {
    if let Some(replacement) = &args.replace {
        replace_in_file(path, re, replacement)
    } else {
        search_in_file(path, re, args)
    }
}

fn search_in_file(path: &Path, re: &Regex, args: &Args) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    for (line_num, line) in reader.lines().enumerate() {
        if let Ok(line_content) = line {
            if let Some(mat) = re.find(&line_content) {
                let path_display = path.display().to_string().bright_blue();
                let line_info = if args.line_numbers {
                    format!("{}:", (line_num + 1).to_string().green())
                } else {
                    "".to_string()
                };

                let start = mat.start();
                let end = mat.end();
                let highlighted = format!(
                    "{}{}{}",
                    &line_content[..start],
                    &line_content[start..end].red().bold(),
                    &line_content[end..]
                );

                println!("{}{} {}", path_display, line_info, highlighted.trim());
            }
        }
    }
    Ok(())
}

fn replace_in_file(path: &Path, re: &Regex, replacement: &str) -> std::io::Result<()> {
    let content = fs::read_to_string(path)?;
    if re.is_match(&content) {
        let new_content = re.replace_all(&content, replacement).to_string();
        
        // Only write if something changed
        if content != new_content {
            fs::write(path, new_content)?;
            println!("{} {}", "Updated:".green().bold(), path.display().to_string().bright_blue());
        }
    }
    Ok(())
}
