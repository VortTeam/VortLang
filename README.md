# VortLang

VortLang is a minimal interpreted programming language written in Rust. Designed with simplicity and learning in mind, it offers clean syntax and fast execution.

![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)

## Features

- **Simple Syntax**: Declare variables with `let` (strings) and `num` (numbers).
- **Formatted Printing**: Use `print(o"...")` with `{variable}` placeholders.
- **Expression Evaluation**: Supports arithmetic operations with keywords like `plus`, `minus`, etc.
- **Error Handling**: Clear error messages for undefined variables or invalid syntax.

## Current Status  

VortLang is in its early stages and currently supports basic features like variable declarations and arithmetic. More functionality (e.g., control flow, functions) is planned for future releases. Stay tuned!  

**Planned Features:**  
- `if`/`else` conditions  
- Loops (`while`, `for`)  
- User-defined functions  
- Lists/arrays
- And more...

## Installation

1. Ensure [Rust and Cargo](https://www.rust-lang.org/tools/install) are installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/VortTeam/VortLang.git
   ```
3. Build the project:
   ```bash
   cd VortLang/lang
   cargo build --release
   ```

## Usage

Run a VortLang script:
```bash
cargo run --release <filename.vl>
```
or
```bash
path\to\vortlang.exe <filename.vl>
```

### Example Script (`hello.vl`)
```rust
let name = "World"
num x = 10 plus 5 times 2
print(o"Hello {name}! Result: {x}")
```

Output:
```
Hello World! Result: 20
```

## Syntax Guide

### Variable Declaration
- **Strings**: Enclose in quotes.
  ```rust
  let message = "Hello"
  ```
- **Numbers**: Use arithmetic expressions.
  ```python
  num result = 5 plus 3 times 2  # Evaluates to 11
  ```

### Print Statements
- **Basic string**:
  ```python
  print("Hello World")
  ```
- **Formatted string** (use `o"..."`):
  ```python
  print(o"Value: {result}")
  ```

### Operators
Use keywords or symbols:
- `plus` or `+`
- `minus` or `-`
- `times`/`multiply` or `*`
- `divide` or `/`

Example:
```python
num calc = 10 divide 2 minus 3  # Evaluates to 2
```

## License

Licensed under [Apache 2.0](LICENSE).

## Contributing

Contributions are welcome! Fork the repository and submit a pull request. For major changes, open an issue first.
