# Advent of Code: Coding Adventures Across Languages

Welcome to the Advent of Code repository—an exhilarating journey designed to enhance coding skills across a myriad of programming languages. Embark on a thrilling exploration of problem-solving through the exciting challenges presented by Advent of Code.

## What is Advent of Code?

[Advent of Code](https://adventofcode.com/) is an annual event that takes place in December, where participants tackle daily coding challenges. Each day leading up to Christmas, a new programming puzzle is released, encouraging participants to explore and enhance their problem-solving skills.

## Project Structure

The repository structure is organized by programming languages and challenge tasks:

```sh
Advent-of-Code/
|-- rust/
|   |-- src/
|       |-- tasks/
|           |-- 2023/
|               |-- 01/
|                   |-- Cargo.toml
|                   |-- src/
|                   |   |-- main.rs
|                       |-- ...
|                   |-- assets/
|                       |-- input.txt
|                       |-- instructions.txt
|                       |-- output.txt
|                       |-- ...
|-- ...
```

### Getting Started

To begin working on the challenges:

1. **Navigate to the Language Folder:** Choose your preferred programming language folder (`python`, `rust`, etc.) in the code folder.
2. **Start Coding:** Each challenge is organized within the corresponding year and day folder. For instance, for Rust challenges of 2023.
3. **Input Storage:** Input files are stored in the `assets` folder within each challenge directory.

### Obtaining the Session Token

Before you start, ensure you have the `AOC_SESSION_TOKEN` environment variable set. Follow these steps to obtain the token:

1. **Access Developer Tools:**
   - Open the developer tools in your browser using either:
      - Windows: `F12` or `Ctrl + Shift + J`
      - MacOS  : `Cmd + Option + I`
   - Alternatively, right-click on the webpage and select "Inspect."

2. **Navigate to the 'Applications' Tab:**
   - Within the developer tools, locate and click on the 'Applications' tab.

3. **Explore the 'Storage' Section:**
   - Look for the 'Storage' section in the left-hand sidebar of the 'Applications' tab.

4. **Access the 'Cookies' Dropdown:**
   - Under 'Storage,' find and click on 'Cookies' in the left-hand sidebar.

5. **Select Advent of Code Website:**
   - In the 'Cookies' dropdown, choose '<https://adventofcode.com/>' to view the site-specific cookies.

6. **Copy Session Token:**
   - Find the 'session' cookie in the list. Copy the value of this cookie; this is your `AOC_SESSION_TOKEN`.

7. **Set Environment Variable:**
   - Set the value of `AOC_SESSION_TOKEN` as an environment variable in a `.env` file at the root of this repository or as a shell environment variable.

This token allows access to the Advent of Code API for retrieving challenge inputs.

### Managing Code

All code management, builds, and executions should be performed from the root of the language folder. For Rust, you can execute code using `cargo run`.

### Roadmap

The roadmap for this project includes:

- [ ] Explore challenges in multiple languages:
  - [x] Rust
  - [ ] Python
  - [ ] TypeScript
  - [ ] Kotlin
  - [ ] Go
  - [ ] Ruby
  - [ ] Scala
  - [ ] Red
  - [ ] Crystal
- [ ] Work on challenges present, past, and future
- [ ] Develop a Frontend Management App for better organization and accessibility.

### Contributing

Contributions are welcome! Feel free to add solutions in different programming languages or improve existing solutions. Ensure your code is organized within the language-specific folders and follows best practices.

### Disclaimer

The repository is designed to explore and practice coding skills. Please refrain from sharing specific challenge solutions, respecting the Advent of Code's spirit of learning and problem-solving.

### Credits

Advent of Code - [Official Website](https://adventofcode.com/)
