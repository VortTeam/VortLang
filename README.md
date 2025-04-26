# VortLang

VortLang is a minimal compiled programming language written in Rust. Designed with simplicity and learning in mind, it offers clean syntax and fast execution.
Its syntax is similar to Python's and Rust's.

![Rust](https://img.shields.io/badge/🦀%20rust-orange?style=for-the-badge)\
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://github.com/VortTeam/)
[![Discord](https://img.shields.io/badge/Discord-Join%20Now-5865F2?logo=discord&logoColor=white)](https://discord.gg/Efe5ws6jcP)
 

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

Licensed under [Apache 2.0 License](LICENSE).

## Contributing

Contributions are welcome! If you are interested in contributing please check [CONTRIBUTING.md](CONTRIBUTING.md)
