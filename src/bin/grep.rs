use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};

fn highlight_pattern(line: &str, pattern: &str) -> String {
    let mut result = String::new(); // mutable variable
    let mut start = 0;
    // Pattern matching with while let
    while let Some(pos) = line[start..].find(pattern) {
        let abs_pos = start + pos;
        result.push_str(&line[start..abs_pos]);
        // Using ANSI escape codes for coloring output
        result.push_str("\x1b[1;33m");
        result.push_str(pattern);
        result.push_str("\x1b[0m");
        start = abs_pos + pattern.len();
    }
    result.push_str(&line[start..]);
    result
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect(); // Collect command-line arguments

    // Error handling
    if args.len() < 3 {
        eprintln!("Usage: grep <pattern> <file>");
        std::process::exit(1);
    }
    
    let pattern = &args[1]; // Borrowing with references
    let filename = &args[2];
    
    let file = fs::File::open(filename)?; // The ? operator for propagating errors, unique to Rust
    let reader = BufReader::new(file); // Buffered reading, using Rust's BufReader
    
    // Enumerate iterator, for (index, value) pairs, idiomatic Rust
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?; // The ? operator for error handling
        if line.contains(pattern) {
            let highlighted = highlight_pattern(&line, pattern);
            println!("{}:{}", line_num + 1, highlighted); // Print with formatting
        }
    }

    Ok(())
}
