# Guessing game

A game where the player has to guess a random number between 1 and 100 in a minimum of moves with instructions " It's more " or " It's less ".

To adding dependencies, simply add a line with the crate (library) name and semantic version in `[dependencies]` of the `Cargo.toml` and run:
```bash
cargo build
```

You can update crates by using the following command. It will rewrite the `Cargo.lock` file:
```bash
cargo update
```

You can open the documentation of the project and installed crates in your browser with this command:
```bash
cargo doc --open
```

## Default type

The default type in Rust is `i32`.