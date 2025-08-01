# Rust Coreutils

*Building Unix utilities in Rust is a fun way to learn the language and its ecosystem.*

While not intended to replace your system utilities, these implementations may offer performance benefits for macOS users. For example, a custom `rm -rf` could soften macOS's System Integrity Protection (SIP) and Finder's cautious file deletion mechanisms by using direct recursive calls, allowing for faster and more flexible file operations. I recognize this might be a controversial approach, but it will also put pressure on me to scope the proper security concerns and study them more intensively. Regardless, I trust myself to handle these responsibly.

## Documentation

- **[TODO.md](docs/TODO.md)** - Planned features and improvements
- **[CONCEPTS.md](docs/CONCEPTS.md)** - Rust concepts learned through implementation