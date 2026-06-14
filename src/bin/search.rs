use anyhow::Context;
use clap::Parser;
use colored::*;
use ignore::WalkBuilder;
use rayon::prelude::*;
use regex::Regex;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "The pattern to search for")]
    pattern: String,

    #[arg(short, long, help = "The replacement string")]
    replace: Option<String>,

    #[arg(default_value = ".", help = "The directory to search in")]
    path: PathBuf,

    #[arg(short, long, help = "Case-insensitive search")]
    ignore_case: bool,

    #[arg(short, long, default_value_t = true, help = "Show line numbers")]
    line_numbers: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let mut regex_builder = regex::RegexBuilder::new(&args.pattern);
    regex_builder.case_insensitive(args.ignore_case);
    
    let re = Arc::new(regex_builder.build().context("Invalid regex pattern")?);

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

    let files: Vec<PathBuf> = WalkBuilder::new(&args.path)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.into_path())
        .collect();

    files.into_par_iter().for_each(|path| {
        if let Err(e) = process_file(&path, &re, &args) {
            eprintln!("{}: {}", "Error processing".red(), path.display());
            eprintln!("  {}", e);
        }
    });

    Ok(())
}

fn process_file(path: &Path, re: &Regex, args: &Args) -> anyhow::Result<()> {
    if let Some(replacement) = &args.replace {
        replace_in_file(path, re, replacement)
    } else {
        search_in_file(path, re, args)
    }
}

fn search_in_file(path: &Path, re: &Regex, args: &Args) -> anyhow::Result<()> {
    let file = File::open(path).context("Failed to open file")?;
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

fn replace_in_file(path: &Path, re: &Regex, replacement: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(path).context("Failed to read file")?;
    if re.is_match(&content) {
        let new_content = re.replace_all(&content, replacement).to_string();
        
        if content != new_content {
            fs::write(path, new_content).context("Failed to write file")?;
            println!("{} {}", "Updated:".green().bold(), path.display().to_string().bright_blue());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_match() {
        let re = Regex::new("rust").unwrap();
        assert!(re.is_match("rust is fast"));
    }

    #[test]
    fn test_regex_no_match() {
        let re = Regex::new("rust").unwrap();
        assert!(!re.is_match("java is okay"));
    }

    #[test]
    fn test_regex_case_insensitive() {
        let mut builder = regex::RegexBuilder::new("rust");
        builder.case_insensitive(true);
        let re = builder.build().unwrap();
        assert!(re.is_match("RUST"));
    }
}
