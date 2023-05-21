# Minigrep

Returns the lines where a string appears in a file.

## Usage

The first argument is the string to search and the second argument is the file name. The environment variable IGNORE_CASE can be added to do a case insensitive search.

```bash
# Search "to" in poem.txt
cargo run -- to poem.txt
IGNORE_CASE=1 cargo run -- to poem.txt
```

You can run tests with `cargo test`.