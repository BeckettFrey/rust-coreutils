# 🦀 Rust Concepts Learned

<details>
<summary><strong>Error Handling</strong></summary>

## The `?` Operator
```rust
let entries = fs::read_dir(path)?; // Propagates errors up the call stack
for entry in entries {
    let entry = entry?; // Unwrap Result, return early if Err
}
```

**💡 Key insight**: The `?` operator is Rust's way of handling errors without explicit match statements. It automatically converts compatible error types and returns early from functions.

## Result vs Panic
- Use `Result<T, E>` for recoverable errors (file not found, invalid input)
- Use `panic!` or `unwrap()` only for programming logic errors
- `expect()` provides better error messages than `unwrap()`

</details>

<details>
<summary><strong>String Types</strong></summary>

## When to Use Each
```rust
let file_name = entry.file_name(); // Returns OsString
let file_name_str = file_name.to_string_lossy(); // Convert for display
```

- **`String`**: Owned, heap-allocated, mutable, UTF-8 guaranteed
- **`&str`**: Borrowed string slice, stack or static, immutable, UTF-8 guaranteed  
- **`OsString`**: Platform-native string encoding (for file paths, env vars)

**💡 Key insight**: Use `OsString` for anything from the operating system, `String` when you need ownership, `&str` for function parameters and temporary references.

</details>

<details>
<summary><strong>Ownership and Borrowing</strong></summary>

## References in Practice
```rust
let pattern = &args[1]; // Borrowing instead of moving
for arg in &args[1..] { // Borrowing the slice
```

**💡 Key insight**: Borrow (`&`) when you don't need to own the data. This avoids unnecessary clones and makes intent clear.

## Iterator Ownership
```rust
for (line_num, line) in reader.lines().enumerate() {
    let line = line?; // Each line is owned by this scope
}
```

</details>

<details>
<summary><strong>Iterator Patterns</strong></summary>

## Chaining Operations
```rust
let args: Vec<String> = env::args().collect();
for arg in &args[1..] { // Skip first element (program name)
```

**💡 Key insight**: Rust iterators are lazy and composable. Operations like `enumerate()`, `skip()`, and `collect()` can be chained efficiently.

## Pattern Matching in Loops
```rust
while let Some(pos) = line[start..].find(pattern) {
    // Process match
}
```

</details>

<details>
<summary><strong>Memory Management</strong></summary>

## Stack vs Heap
- String literals (`"hello"`) live in the binary, referenced by `&str`
- `String::new()` allocates on the heap, grows as needed
- `Vec<String>` stores String handles on stack, data on heap

## Avoiding Allocations
```rust
// Good: reuses string capacity
result.push_str(&line[start..abs_pos]);

// Less efficient: creates new string each time  
result = result + &line[start..abs_pos];
```

</details>

<details>
<summary><strong>Command-Line Parsing</strong></summary>

## Manual vs Libraries
Current approach uses manual parsing:
```rust
for arg in &args[1..] {
    if arg == "-a" {
        show_all = true;
    } else {
        path = arg;
    }
}
```

**💡 Trade-offs**: Manual parsing is simple for basic flags but becomes unwieldy with complex options. Libraries like `clap` provide better error messages and help text.

</details>

<details>
<summary><strong>File I/O Patterns</strong></summary>

## Buffered Reading
```rust
let file = fs::File::open(filename)?;
let reader = BufReader::new(file);
```

**💡 Key insight**: `BufReader` reduces system calls by reading chunks into memory. Essential for good performance when processing line-by-line.

## Path Handling
```rust
let file_name = entry.file_name(); // OsString, not String
```

File names aren't guaranteed to be valid UTF-8 on all systems, so Rust uses `OsString` to handle platform differences safely.

</details>

<details>
<summary><strong>ANSI Terminal Codes</strong></summary>

## Color Output
```rust
result.push_str("\x1b[1;33m"); // Yellow bold
result.push_str(pattern);
result.push_str("\x1b[0m");    // Reset
```

**💡 Key insight**: Terminal colors are just escape sequences. Rust's string handling makes it easy to build formatted output programmatically.

</details>

<details>
<summary><strong>Learning Resources Used</strong></summary>

- [Interactive Rust Book](https://rust-book.cs.brown.edu/) - Excellent for understanding ownership
- [The Rust Programming Language](https://doc.rust-lang.org/book/) - Comprehensive reference
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical code patterns
- GNU coreutils source code - Understanding edge cases and expected behavior

</details>