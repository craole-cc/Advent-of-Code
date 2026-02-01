# Contributing Guidelines

Thank you for your interest in contributing to this Advent of Code repository! This document provides guidelines and best practices for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Solution Guidelines](#solution-guidelines)
- [Code Style](#code-style)
- [Adding New Languages](#adding-new-languages)
- [Improving Documentation](#improving-documentation)
- [Submitting Changes](#submitting-changes)

## Code of Conduct

### Be Respectful

- **No spoilers** - Don't share solutions before the leaderboard closes
- **Be helpful** - If discussing approaches, guide without revealing answers
- **Be constructive** - Provide helpful feedback on code reviews
- **Respect AoC** - Follow Eric Wastl's guidelines for the event

### Advent of Code Guidelines

From the [AoC about page](https://adventofcode.com/about):

- Don't publish your inputs (they're personalized)
- Don't automate puzzle submissions
- Be kind to the servers (reasonable rate limiting)
- Give credit when sharing derivative works

## How to Contribute

### Types of Contributions

We welcome:

- ✅ **Solutions** - In any programming language
- ✅ **Optimizations** - Faster or more elegant implementations
- ✅ **Documentation** - Improving guides and explanations
- ✅ **Tooling** - Scripts, utilities, or infrastructure improvements
- ✅ **Tests** - Additional test cases or validation
- ✅ **Bug fixes** - Corrections to existing code
- ✅ **New languages** - Implementations in currently unsupported languages

### Getting Started

1. **Fork the repository**
2. **Create a branch** - `git checkout -b feature/rust-2024-05`
3. **Make your changes**
4. **Test thoroughly**
5. **Submit a pull request**

## Solution Guidelines

### When Adding Solutions

**DO:**

- ✅ Place solutions in the correct language/year/day directory
- ✅ Use the shared `assets/` directory for inputs
- ✅ Include comments explaining complex logic
- ✅ Add tests using the example inputs
- ✅ Follow the language's idioms and best practices
- ✅ Keep solutions self-contained (minimize dependencies)

**DON'T:**

- ❌ Commit puzzle inputs to git (they're personal)
- ❌ Hard-code answers in the source
- ❌ Include solutions before the day's leaderboard closes
- ❌ Copy solutions without attribution
- ❌ Over-engineer simple problems

### File Structure

Each solution should follow this pattern:

```sh
code/<language>/<year>/<day>/
├── Cargo.toml / package.json / etc.  # If needed
├── src/
│   └── main.<ext>                     # Your solution
└── tests/                             # Optional but encouraged
    └── test.<ext>
```

### Solution Template

A good solution includes:

```rust
// Example for Rust
use std::fs;

fn main() {
    let input = fs::read_to_string("../../../assets/2024/05/input.txt")
        .expect("Failed to read input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    // Clear, commented solution
    todo!()
}

fn part2(input: &str) -> i32 {
    // Clear, commented solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 42);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 84);
    }
}
```

## Code Style

### General Principles

- **Clarity over cleverness** - Readable code is better than terse code
- **Comments for why, not what** - Explain reasoning, not obvious operations
- **Consistent formatting** - Use language-standard formatters
- **Meaningful names** - Use descriptive variable and function names

### Language-Specific Style

#### Rust

- Use `rustfmt` - `cargo fmt`
- Use `clippy` - `cargo clippy`
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Prefer iterators over loops when appropriate

```sh
# Before committing
cargo fmt
cargo clippy
cargo test
```

#### Shell Script (POSIX)

- Use [ShellCheck](https://www.shellcheck.net/) for linting
- Follow POSIX standards (avoid bash-isms)
- Use `set -e` and `set -u` at script start
- Quote all variables

```sh
# Before committing
shellcheck script.sh
sh -n script.sh  # Check syntax
```

#### Zig

- Use `zig fmt` for formatting
- Follow [Zig Style Guide](https://ziglang.org/documentation/master/#Style-Guide)
- Prefer explicit over implicit

```sh
# Before committing
zig fmt .
zig build test
```

#### Python (when implemented)

- Follow [PEP 8](https://pep8.org/)
- Use `black` for formatting - `black .`
- Use `pylint` or `ruff` for linting
- Type hints appreciated but not required

```sh
# Before committing
black .
pylint src/
pytest
```

#### Go (when implemented)

- Use `go fmt` for formatting
- Use `golint` or `staticcheck` for linting
- Follow [Effective Go](https://go.dev/doc/effective_go)

```sh
# Before committing
go fmt ./...
go vet ./...
go test ./...
```

#### TypeScript (when implemented)

- Use `prettier` for formatting
- Use `eslint` for linting
- Follow project's tsconfig.json settings

```sh
# Before committing
npm run format
npm run lint
npm test
```

## Adding New Languages

Want to add support for a new language? Great! Here's how:

### 1. Create Language Structure

```sh
mkdir -p code/<language>/
cd code/<language>/
```

### 2. Set Up Build System

Create appropriate build configuration:

- Rust: `Cargo.toml` workspace
- Python: `pyproject.toml` or `setup.py`
- Zig: `build.zig`
- Node: `package.json`

### 3. Create Year/Day Structure

```sh
mkdir -p <year>/<day>/src
```

### 4. Reference Shared Assets

Your code should read from:

```sh
assets/<year>/<day>/input.txt
```

Adjust path based on your directory depth.

### 5. Add Documentation

Create `code/<language>/README.md` with:

- Setup instructions
- How to run solutions
- Dependencies
- Language-specific tips

### 6. Update Main README

Add your language to the main README's supported languages list.

## Improving Documentation

Documentation improvements are always welcome!

### What to Document

- Common algorithms and their applications
- Tricky edge cases in specific puzzles
- Performance optimization techniques
- Setup troubleshooting
- Language-specific tips

### Documentation Style

- **Clear headings** - Use markdown headers appropriately
- **Code examples** - Show, don't just tell
- **Links** - Reference official docs where helpful
- **Screenshots** - For UI/setup instructions (sparingly)

### Where to Add Documentation

- `documentation/` - General guides
- `code/<language>/README.md` - Language-specific
- Inline comments - Complex algorithms
- Commit messages - Why changes were made

## Submitting Changes

### Before Submitting

**Checklist:**

- [ ] Code follows style guidelines
- [ ] Tests pass (`cargo test`, `pytest`, etc.)
- [ ] No compiler warnings
- [ ] Puzzle input NOT committed
- [ ] Documentation updated if needed
- [ ] Commit messages are clear

### Pull Request Process

1. **Update your fork:**

```sh
git remote add upstream https://github.com/original/repo.git
git fetch upstream
git rebase upstream/main
```

1. **Create descriptive PR:**

- Title: `Add Rust solution for 2024 day 5`
- Description: Explain approach, complexity, any interesting insights

1. **Link related issues:**

- Mention issue numbers if applicable
- Use keywords: "Fixes #123" or "Closes #456"

1. **Respond to feedback:**

- Be open to suggestions
- Make requested changes
- Ask questions if unclear

### Commit Message Format

Good commit messages help others understand your changes:

```md
Add solution for 2024 day 5 in Rust

- Implemented BFS algorithm for part 1
- Used dynamic programming for part 2
- Both parts run in O(n log n) time
```

**Format:**

- First line: Brief summary (50 chars or less)
- Blank line
- Detailed explanation if needed

## Testing

### Required Tests

- **Example tests** - Use provided examples from puzzle descriptions
- **Edge cases** - Empty inputs, boundary values, etc.

### Optional Tests

- **Performance tests** - Benchmarks for optimization PRs
- **Property-based tests** - For algorithmic correctness
- **Integration tests** - Full solution validation

### Running Tests

```sh
# Rust
cargo test

# Python
pytest

# Zig
zig build test
```

## Performance Considerations

### When Optimization Matters

- Solutions that take >10 seconds to run
- Part 2 that scales significantly from part 1
- When demonstrating algorithmic improvements

### Optimization Guidelines

1. **Measure first** - Profile before optimizing
2. **Document tradeoffs** - Note complexity vs. readability
3. **Keep simple version** - Comment out or commit separately
4. **Benchmark changes** - Show improvement metrics

## Recognition

Contributors will be:

- Listed in commit history (automatic)
- Mentioned in release notes (for significant contributions)
- Credited in solution files (for novel approaches)

## Questions?

- **Open an issue** - For questions or discussions
- **Check existing issues** - Someone may have asked already
- **Join the discussion** - Comment on relevant issues/PRs

## License

By contributing, you agree that your contributions will be licensed under the same license as this project (see LICENSE file).

---

> Every contribution, **no matter how small**, helps make this project better for everyone learning through Advent of Code.
