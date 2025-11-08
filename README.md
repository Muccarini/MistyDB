# MistyDB & Mu Language

A database system with a scripting language for automation.

## Quick Start

```bash
# Build
cargo build --release

# Run MistyDB
cargo run --bin misty-db

# Run Mu script
cargo run --bin mu -- run script.mu

# Interactive REPL
cargo run --bin mu -- repl
```

## Mu Language

### Basics
```mu
// Variables
let x = 10;
let name = "MistyDB";
let active = true;

// Functions
func greet(name) {
    "Hello, " + name;
}

let msg = greet("World");
```

### Example
```mu
func calculate_area(width, height) {
    width * height;
}

let result = calculate_area(10, 20);
```