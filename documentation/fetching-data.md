# Advent of Code - Data Fetcher

A POSIX-compliant shell script to automatically fetch Advent of Code puzzle inputs, instructions, and organize them in a structured directory format.

## Features

- ✅ **POSIX-compliant** - Works on any Unix-like system (Linux, macOS, BSD, WSL)
- ✅ **Interactive mode** - User-friendly menu interface
- ✅ **Multiple fetch modes** - Single day, latest puzzle, or all missing data
- ✅ **Smart validation** - Prevents fetching unreleased puzzles
- ✅ **Rate limiting protection** - Automatic retries with exponential backoff
- ✅ **Time zone aware** - Respects AoC's EST release schedule (00:00 EST / 05:00 UTC)
- ✅ **Project-aware** - Automatically finds project root
- ✅ **Robust error handling** - Clear error messages and graceful failures

## Prerequisites

- **curl** (for HTTP requests)
- **git** (for project root detection)
- **A valid Advent of Code session cookie**

## Quick Start

1. **Get your session cookie:**
    - Log in to [Advent of Code](https://adventofcode.com)
    - Open browser Developer Tools (F12)
    - Navigate to: Storage → Cookies → `https://adventofcode.com`
    - Copy the value of the `session` cookie

2. **Set up the session key:**

    ```sh
    mkdir -p .env
    echo "YOUR_SESSION_COOKIE_HERE" > .env/session.key
    chmod 600 .env/session.key
    ```

    ⚠️ Important: The .env/session.key file is gitignored. Never commit this file!

3. **Make the script executable**

    ```sh
    chmod +x fetch-aoc.sh
    ```

## Usage

### Interactive Mode (Recommended)

Menu Options:

1) Latest puzzle (currently: 2024 day 10)
2) Specific day of current year (2024)
3) Specific year and day
4) All missing data (2015-2024)
5) Show help
6) Exit

```sh
fetcher=""${PRJ_ROOT:-$(pwd -p)}/scripts/fetch-aoc.sh"
sh "${fetcher}"
```

### Command-Line Mode

```sh
sh "${fetcher}" [<year> <day> | <day> | all | latest]
```

### Examples

```sh
sh "${fetcher}"         # Interactive mode
sh "${fetcher}" latest  # Fetch the last available puzzle
sh "${fetcher}" all     # Fetch all missing data (2015-current)
sh "${fetcher}" 5       # Fetch day 5 of the current year
sh "${fetcher}" 2023 5  # Fetch day 5 of 2023
```

## Directory Structure

The script organizes fetched data in this structure:
text

```sh
assets/
└── data/
    ├── 2015/
    │   ├── 01/
    │   │   ├── input.txt        # Puzzle input
    │   │   ├── instructions.txt # Problem description (HTML)
    │   │   └── output.txt       # Placeholder for your solution
    │   ├── 02/
    │   └── .../
    ├── 2016/
    └── .../
```

## How It Works

### Date Validation

- Prevents fetching unreleased puzzles (checks EST time zone)
- Validates year range (2015-current year)
- Checks day range (1-25, or current day during December)

### Session Management

- Checks SESSION_KEY environment variable
- Falls back to .env/session.key file
- Validates session before fetching

### Rate Limiting Protection

- Implements exponential backoff (1s, 2s, 4s...)
- Handles HTTP 429 (Too Many Requests) responses
- Respects server errors (500, 502, 503, 504)
- Adds random delays (2-5s) between batch requests

### Error Messages

The script provides clear error messages for common issues:

- "Session key not found" - Missing or invalid session cookie
- "Day X hasn't been released yet!" - Attempting to fetch future puzzle
- "HTTP 429: Rate limited" - Too many requests (automatic retry)
- "HTTP 401: Invalid or expired session key" - Session cookie needs renewal

## Configuration

### Environment Variables

- **SESSION_KEY**: Advent of Code session cookie (alternative to file)

## Project Structure

The script automatically detects the project root by:

- Looking for a .git directory (if in a git repository)
- Falling back to the script's directory

## File Locations

- **Session Key**: .env/session.key (or SESSION_KEY environment variable)
- **Data Directory**: assets/data/ (relative to project root)

## Best Practices

- Keep session key secure: Never commit or share your session cookie
- Respect rate limits: Use all mode sparingly; it adds delays between requests
- Check puzzle availability: Remember puzzles release at 00:00 EST (05:00 UTC)
- Renew session key: Session cookies expire after ~30 days
- Back up your data: Consider committing fetched puzzles to version control

## Troubleshooting

### Common Issues

> "Session key not found"

- Solution 1: Set environment variable

  ```sh
  export SESSION_KEY="your_session_cookie_here"
  ```

- Solution 2: Create session key file

  ```sh
  mkdir -p .env
  echo "your_session_cookie_here" > .env/session.key
  chmod 600 .env/session.key
  ```

> "Day X hasn't been released yet!"

- Check current EST time: date -d 'TZ="EST" +%Y-%m-%d\ %H:%M'
- Puzzles release at 00:00 EST (05:00 UTC)
- During December, only current and past days are available

> "HTTP 429: Rate limited"

- Wait a few minutes and try again
- The script automatically retries 3 times with backoff
- Consider using longer delays for batch operations

> "Failed to fetch puzzle"

- Verify internet connectivity
- Check if Advent of Code is accessible
- Ensure your session cookie is still valid (re-login if needed)

## Debug Mode

For detailed output, run with shell debugging:

```bash
bash -x fetch-aoc.sh [arguments]
```

## Compatibility

- Shells: Any POSIX-compliant shell (bash, dash, ksh, zsh)
- Systems: Linux, macOS, BSD, WSL, Cygwin
- Dependencies: Only curl (and git for project root detection)

## Contributing

Feel free to submit issues or pull requests for:

- Bug fixes
- Feature enhancements
- Documentation improvements
- Compatibility updates

## License

This project is licensed under the [MIT License](../LICENSE).

**Note:** The [Advent of Code](https://adventofcode.com/) puzzle data fetched by this script remains the property of ([Eric Wastl](http://was.tl/)). This tool simply helps automate access to data you've already unlocked through your account.
