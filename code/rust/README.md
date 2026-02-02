# Advent of Code: Rust Edition

Welcome to the Rust edition of Advent of Code! This repository is dedicated to
honing Rust programming skills through the exploration of Advent of Code
challenges.

## Structure

```sh
.
├── .cargo
│   └── config.toml
├── .gitignore
├── .mise.toml
├── Cargo.lock
├── Cargo.toml
├── README.md
├── crates
│   ├── 2023
│   │   ├── 01
│   │   │   ├── Cargo.toml
│   │   │   └── src
│   │   │       └── main.rs
│   │   ├── 02
│   │   ├── 03
│   │   ├── 04
│   │   ├── 05
│   │   ├── 06
│   │   ├── 07
│   │   ├── 08
│   │   ├── 09
│   │   ├── 10
│   │   ├── 11
│   │   └── 12
│   ├── 2024
│   ├── 2025
│   └── admin
│       ├── Cargo.toml
│       └── src
│           ├── cli
│           │   └── mod.rs
│           ├── env
│           │   ├── local.rs
│           │   └── mod.rs
│           ├── error
│           │   ├── mod.rs
│           │   └── this.rs
│           ├── hook
│           │   ├── generate_pkg_name.rs
│           │   └── mod.rs
│           ├── jobs
│           │   ├── getters.rs
│           │   ├── mod.rs
│           │   └── setters.rs
│           ├── lib.rs
│           ├── main.rs
│           ├── types
│           │   ├── aoc.rs
│           │   ├── environment.rs
│           │   ├── mod.rs
│           │   └── package.rs
│           └── utils
│               ├── day.rs
│               ├── mod.rs
│               └── print.rs
├── flake.lock
├── flake.nix
└── treefmt.toml
```

## Quick Start

```sh
#> Create a new crate
mise run new-crate my-crate

#> Run tests for all crates
cargo test --workspace

#> Run a specific crate
cargo run -p crate-name
```

1. **Run Challenges:** Execute challenges using `cargo run`.
2. **Input Storage:** Input files are stored in the `assets` folder within each
   challenge directory.

## Development

- **Watch mode**: `mise run dev`
- **Test all**: `mise run test`
- **Format**: `mise run fmt`
- **Check**: `mise run check`

## License

MIT OR Apache-2.0
