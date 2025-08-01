use std::env; // For accessing command-line arguments
use std::fs;  // For filesystem operations like reading directories
use std::io;  // For input/output types, including Result

fn main() -> io::Result<()> { // Main returns a Result to handle errors gracefully

    let args: Vec<String> = env::args().collect();

    let mut show_all = false; 
    let mut path = ".";       

    // Iterate over arguments, skipping the first (program name)
    for arg in &args[1..] {
        if arg == "-a" {      
            show_all = true;
        } else {
            path = arg;
        }
    }

    let entries = fs::read_dir(path)?; 
    for entry in entries {
        let entry = entry?; // Unwrap Result, propagating errors if any
        let file_name = entry.file_name(); // Get file name as OsString
        let file_name_str = file_name.to_string_lossy(); // Convert OsString to String for display
        if show_all || !file_name_str.starts_with('.') {
            println!("{}", file_name_str); 
        }
    }
    Ok(()) // Indicate successful completion
}
