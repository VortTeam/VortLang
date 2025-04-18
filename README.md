# VortLang

VortLang is a minimal compiled programming language written in Rust. Designed with simplicity and learning in mind, it offers clean syntax and fast execution.
Its syntax is similar to Python's and Rust's.

![Rust](https://img.shields.io/badge/🦀%20rust-orange?style=for-the-badge)
[![License](https://img.shields.io/badge/License-VORTTEAM%20GITHUB%20LICENSE%20v1-blueviolet?style=for-the-badge)](https://github.com/VortTeam/)\
[![Discord](https://img.shields.io/badge/Discord-Join%20Now-5865F2?logo=discord&logoColor=white)](https://discord.gg/At3CcCqcR2)


## Features

- **Simple Syntax**: Declare variables with `let` (strings) and `num` (numbers).
- **Formatted Printing**: Use `print(o"...")` with `{variable}` placeholders.
- **Expression Evaluation**: Supports arithmetic operations with keywords like `plus`, `minus`, etc.
- **Error Handling**: Clear error messages for undefined variables or invalid syntax.

## Current Status  

VortLang is in its early stages and currently supports basic features like variable declarations and arithmetic. More functionality is planned for future releases. Stay tuned!  

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
cargo run --release <path/to/filename.vl>
```
or
```bash
path/to/vortlang.exe <filename.vl>
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
## Documentation
[here](/doc)

## License

Licensed under [VORTTEAM GITHUB LICENSE VERSION 1.0](LICENSE).

## Contributing

Contributions are welcome! If you are interested in contributing please check [CONTRIBUTING.md](CONTRIBUTING.md)
