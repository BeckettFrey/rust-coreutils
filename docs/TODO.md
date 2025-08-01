# TODO

## Immediate Improvements

### grep
- [ ] Add actual regex support (currently just string matching)
- [ ] Add context lines (`-C`, `-A`, `-B` flags)
- [ ] Support multiple files and recursive directory search
- [ ] Add case-insensitive matching (`-i` flag)
- [ ] Improve error messages for invalid patterns

### ls
- [ ] Show file permissions and ownership
- [ ] Display file sizes with human-readable formatting
- [ ] Add long format listing (`-l` flag)
- [ ] Sort by modification time (`-t` flag)
- [ ] Add color coding for different file types

## New Utilities

### Next Up
- [ ] **`cat`** - File concatenation and display
- [ ] **`wc`** - Word, line, and character counting
- [ ] **`head`/`tail`** - Show beginning/end of files

### Future Implementations
- [ ] **`find`** - File search with predicates
- [ ] **`sort`** - Line sorting with different criteria
- [ ] **`uniq`** - Remove duplicate lines
- [ ] **`cut`** - Extract columns from text

## Technical Improvements

### Performance
- [ ] Benchmark against GNU coreutils
- [ ] Profile memory usage patterns
- [ ] Optimize string handling in grep
