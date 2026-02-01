# Advent of Code - Data Fetcher

POSIX-compliant shell scripts to automatically fetch Advent of Code puzzle inputs and instructions.

## Prerequisites

- `curl` (should be available on most systems)
- A valid Advent of Code session cookie

## Setup

### 1. Get Your Session Cookie

1. Log in to [Advent of Code](https://adventofcode.com/<year>/auth/login)
2. Open your browser's developer tools (F12)
3. Go to the `Storage` tab
4. Access `Cookies > https://adventofcode.com` from the storage list
5. Copy the value of the `session` cookie

### 2. Save Your Session Key

Create the session key file:

```sh
mkdir -p .env
echo "YOUR_SESSION_COOKIE_HERE" > .env/session.key
chmod 600 .env/session.key
```

**Important:** The `session.key` file is gitignored. Never commit this file!

## Usage

All scripts are located in the `scripts/` directory at the project root.

### Fetch a Single Day

```sh
./scripts/fetch-aoc.sh [<year> <day> | <day>]
```

Examples:

```sh
./scripts/fetch-aoc.sh           # the latest day
./scripts/fetch-aoc.sh 5         # day 5 of the latest year
./scripts/fetch-aoc.sh 2023 1    # day 1 of 2023
./scripts/fetch-aoc.sh 2024 15   # day 15 of 2024
```

### Fetch Multiple Days

```sh
./scripts/fetch-aoc-batch.sh <year> [start_day] [end_day]
```

Examples:

```sh
./scripts/fetch-aoc-batch.sh 2023      # all days of 2023
./scripts/fetch-aoc-batch.sh 2023 1 5  # days 1-5 of 2023
./scripts/fetch-aoc-batch.sh 2023 10   # day 10 of 2023
```

## Directory Structure

After fetching, assets are organized as:

```sh
assets
├── data
│   ├── <year>
│   │   ├── <day>
│   │   │   ├── input.txt # Your puzzle input
│   │   │   ├── instructions.txt # Problem description (HTML)
│   │   │   └── output.txt # Placeholder for your solution
│   │   └── ... # Other days
│   └── ... # Other years
└── ... # Other assets
```

## Features

- ✅ POSIX-compliant (works on Linux, macOS, BSD, WSL)
- ✅ Uses native `curl` (no language-specific dependencies)
- ✅ Validates year (2015-2025) and day (1-25)
- ✅ Creates directory structure automatically
- ✅ Respects AoC servers with delays between batch requests
- ✅ Error handling and helpful messages
- ✅ Smart argument parsing:
  - No args → fetch latest puzzle
  - One arg → fetch that day from latest year
  - Two args → fetch specific year and day

## Notes

- The session cookie typically expires after ~30 days
- If fetching fails, check that your session key is still valid
- Instructions are saved as HTML - you may want to parse them further
- The batch script adds a 1-second delay between requests to be respectful to AoC servers
- Puzzles release at midnight EST (05:00 UTC) each day during December

## Troubleshooting

> "Session key file not found"

- Ensure the key is saved at `.env/session.key`

> "Failed to fetch input"**

- Check that your session cookie is still valid
- Verify you're connected to the internet
- Make sure the puzzle has been released (after midnight EST on the given day)

> "curl is required but not installed"

- Install curl: `apt install curl` (Debian/Ubuntu) or `brew install curl` (macOS)

> "Day X hasn't been released yet!"

- Check the current EST time - puzzles release at midnight EST (05:00 UTC)
- If it's December, wait until after the release time
- If it's before December, only previous years' puzzles are available
