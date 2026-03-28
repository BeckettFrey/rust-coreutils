use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};

fn count_file(filename: &str) -> io::Result<(usize, usize, usize)> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;

    // Process line by line using buffered reader
    for line in reader.lines() {
        let line = line?;
        lines += 1;
        bytes += line.len() + 1; // +1 for the newline character
        // split_whitespace handles multiple spaces, tabs, etc.
        words += line.split_whitespace().count();
    }

    Ok((lines, words, bytes))
}

fn count_stdin() -> io::Result<(usize, usize, usize)> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock()); // Lock stdin for buffered reading

    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;

    for line in reader.lines() {
        let line = line?;
        lines += 1;
        bytes += line.len() + 1;
        words += line.split_whitespace().count();
    }

    Ok((lines, words, bytes))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Parse flags
    let mut show_lines = false;
    let mut show_words = false;
    let mut show_bytes = false;
    let mut files: Vec<&str> = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-l" => show_lines = true,
            "-w" => show_words = true,
            "-c" => show_bytes = true,
            _ => files.push(arg),
        }
    }

    // If no flags specified, show all (default behavior matching GNU wc)
    if !show_lines && !show_words && !show_bytes {
        show_lines = true;
        show_words = true;
        show_bytes = true;
    }

    if files.is_empty() {
        // Read from stdin when no files given
        let (lines, words, bytes) = count_stdin()?;
        print_counts(lines, words, bytes, show_lines, show_words, show_bytes, None);
    } else {
        let mut total_lines = 0;
        let mut total_words = 0;
        let mut total_bytes = 0;

        for filename in &files {
            let (lines, words, bytes) = count_file(filename)?;
            total_lines += lines;
            total_words += words;
            total_bytes += bytes;
            print_counts(lines, words, bytes, show_lines, show_words, show_bytes, Some(filename));
        }

        // Print total if more than one file
        if files.len() > 1 {
            print_counts(total_lines, total_words, total_bytes, show_lines, show_words, show_bytes, Some("total"));
        }
    }

    Ok(())
}

fn print_counts(lines: usize, words: usize, bytes: usize, show_lines: bool, show_words: bool, show_bytes: bool, label: Option<&str>) {
    let mut parts: Vec<String> = Vec::new();
    if show_lines {
        parts.push(format!("{:>8}", lines));
    }
    if show_words {
        parts.push(format!("{:>8}", words));
    }
    if show_bytes {
        parts.push(format!("{:>8}", bytes));
    }
    match label {
        Some(name) => println!("{} {}", parts.join(""), name),
        None => println!("{}", parts.join("")),
    }
}
