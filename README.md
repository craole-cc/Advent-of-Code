# Advent of Code: Coding Adventures Across Languages

Welcome to the Advent of Code repository—an exhilarating journey designed to enhance coding skills across a myriad of programming languages. Embark on a thrilling exploration of problem-solving through the exciting challenges presented by Advent of Code.

## What is Advent of Code?

[Advent of Code](https://adventofcode.com/) is an annual event that takes place in December, where participants tackle daily coding challenges. Each day leading up to Christmas, a new programming puzzle is released, encouraging participants to explore and enhance their problem-solving skills.

## Quick Start

```sh
# 1. Clone the repository
git clone https://github.com/craole-cc/Advent-of-Code.git
cd Advent-of-Code

# 2. Set up your session key
mkdir -p .env
echo "YOUR_SESSION_COOKIE" > .env/session.key

# 3. Fetch AoC data
./scripts/fetch-aoc.sh

# 4. Start coding!
language='rust'   # or shellscript, python, zig, etc.
cd "code/${language}"
```

## Project Structure

```sh
Advent-of-Code/
├── assets    # Shared puzzle data
├── code      # Solutions
│   ├── rust
│   ├── python
│   ├── shellscript
│   ├── zig
│   └── ... # Other languages
├── documentation
├── scripts   # Common utilities
└── README.md
```

## Documentation

### Project-specific

- 📚 [Getting Started](documentation/getting-started.md) - Setup and first steps
- 🔍 [Fetching Assets](documentation/fetching-data.md) - How to download puzzle inputs
- 🤝 [Contributing](CONTRIBUTING.md) - How to contribute

### Language-specific

- [x] 🦀 [Rust](code/rust/README.md)
- [ ] 🐚 [ShellScript](code/shellscript/README.md)
- [ ] 🐍 Python
- [ ] ⚡ Zig
- [ ] TypeScript
- [ ] Go
- [ ] Other languages - Open to contributions!

## Quick Examples

### Fetch the most recent puzzle

```sh
./scripts/fetch-aoc.sh 2024 latest
```

### Fetch all puzzles for a year

```sh
./scripts/fetch-aoc.sh 2023
```

### Run a Rust solution

```sh
cd code/rust
cargo run --bin 2024-01
```

## Features

- ✨ **Language-Agnostic Assets** - Fetch once, use everywhere
- 🚀 **POSIX Scripts** - Works on Linux, macOS, BSD, WSL
- 📦 **Organized Structure** - Clear separation of concerns
- 🔧 **CLI Tools** - Admin utilities for project management
- 🎯 **Zero Dependencies** - Core scripts use only `curl`

## Community & Support

- 🐛 [Report Issues](../../issues)
- 💡 [Request Features](../../issues/new)
- 🌟 [Star this repo](../../stargazers) if you find it helpful!

## Credits

- **[Advent of Code](https://adventofcode.com/)** by [Eric Wastl](http://was.tl/)

## License

This project is open source. Please respect the Advent of Code community guidelines:

- Don't spoil puzzles for others
- Don't automate submissions
- Credit Advent of Code when sharing

---

> *"The best way to learn is by doing."*
