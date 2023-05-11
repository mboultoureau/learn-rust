# Hello cargo

Build the project and execute it:
```bash
cargo build
./target/debug/hello_cargo
```

Or simply run this command to compile and run:
```bash
cargo run
```

We can use this command to check if the code compiles without producing an executable:
```bash
cargo check
```

To prouce an executable with optimizations and run it, use the following commands:
```bash
cargo build --release
./target/release/hello_cargo
```