# Advent of Code: POSIX ShellScript Edition

This guide covers solving Advent of Code challenges using POSIX-compliant shell scripts.

## Why Shell Script for AoC?

Shell scripting is an excellent choice for certain AoC puzzles because:

- 🔧 **Text processing powerhouse** - Excellent at parsing and manipulating text
- 🚀 **Quick prototyping** - Fast to write and iterate
- 📦 **No dependencies** - Works everywhere Unix-like systems exist
- 🎓 **Learn fundamentals** - Deepens understanding of Unix philosophy
- ⚡ **Leverage Unix tools** - `awk`, `sed`, `grep`, `cut`, `sort`, etc.

**Best for:** String manipulation, pattern matching, file processing
**Challenging for:** Complex algorithms, heavy computation, large data structures

## Project Structure

```sh
code/shellscript/
├── README.md           # Shell-specific documentation
├── lib/               # Shared utility functions
│   ├── common.sh     # Common functions
│   ├── parse.sh      # Parsing utilities
│   └── math.sh       # Math helpers
└── <year>/
    └── <day>/
        ├── part1.sh  # Part 1 solution
        ├── part2.sh  # Part 2 solution
        └── test.sh   # Tests (optional)
```

## Getting Started

### Prerequisites

Standard Unix tools (usually pre-installed):

- **POSIX shell** - `sh`, `dash`, `bash` (in POSIX mode)
- **Core utilities** - `awk`, `sed`, `grep`, `cut`, `tr`, `sort`, `uniq`
- **bc** - For floating-point arithmetic (optional)

### Verify Your Environment

```sh
# Check shell
sh --version

# Check utilities
which awk sed grep cut tr sort uniq

# Optional: Check bc
which bc
```

## Creating a Solution

### Basic Template

```sh
#!/bin/sh
# Advent of Code <YEAR> - Day <DAY> - Part <N>
# Description: <Brief description>

set -e  # Exit on error
set -u  # Error on undefined variables

# Configuration
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
INPUT_FILE="${SCRIPT_DIR}/../../../assets/<YEAR>/<DAY>/input.txt"

# Main solution
solve() {
    local input_file="$1"

    # Your solution here

    echo "$result"
}

# Entry point
main() {
    if [ ! -f "$INPUT_FILE" ]; then
        echo "Error: Input file not found: $INPUT_FILE" >&2
        exit 1
    fi

    result=$(solve "$INPUT_FILE")
    echo "Answer: $result"
}

# Run if executed directly (not sourced)
if [ "${BASH_SOURCE[0]:-$0}" = "$0" ]; then
    main "$@"
fi
```

### Example: 2023 Day 1 Part 1

Problem: Extract first and last digit from each line, combine them.

```sh
#!/bin/sh
# AoC 2023 Day 1 Part 1 - Trebuchet?!

set -e

INPUT_FILE="../../../assets/2023/01/input.txt"

solve() {
    local total=0

    while IFS= read -r line; do
        # Extract only digits
        digits=$(echo "$line" | tr -cd '0-9')

        # Get first and last digit
        first=$(echo "$digits" | cut -c1)
        last=$(echo "$digits" | rev | cut -c1)

        # Combine and add to total
        number="${first}${last}"
        total=$((total + number))
    done < "$1"

    echo "$total"
}

result=$(solve "$INPUT_FILE")
echo "Part 1: $result"
```

## Common Patterns

### Reading Input Line by Line

```sh
while IFS= read -r line; do
    echo "Processing: $line"
done < "$INPUT_FILE"
```

### Processing CSV/Delimited Data

```sh
# Using cut
while IFS= read -r line; do
    field1=$(echo "$line" | cut -d',' -f1)
    field2=$(echo "$line" | cut -d',' -f2)
    echo "$field1 -> $field2"
done < input.txt

# Using awk
awk -F',' '{print $1, $2}' input.txt
```

### Extracting Numbers

```sh
# All numbers from a line
numbers=$(echo "abc123def456" | grep -o '[0-9]\+')

# Just digits
digits=$(echo "a1b2c3" | tr -cd '0-9')

# Using sed
number=$(echo "Value: 42" | sed 's/[^0-9]//g')
```

### Pattern Matching

```sh
# Check if line matches pattern
if echo "$line" | grep -q "pattern"; then
    echo "Match found"
fi

# Extract matched groups (using sed)
value=$(echo "x=123" | sed 's/x=\([0-9]*\)/\1/')
```

### Counting and Summing

```sh
# Count lines
count=$(wc -l < input.txt)

# Sum numbers in a column
total=$(awk '{sum += $1} END {print sum}' input.txt)

# Count occurrences
count=$(grep -c "pattern" input.txt)
```

### Sorting and Uniqueness

```sh
# Sort numerically
sort -n input.txt

# Sort and get unique values
sort input.txt | uniq

# Count unique values
sort input.txt | uniq | wc -l
```

### Arrays (Portably)

```sh
# Space-separated (be careful with spaces in values)
items="apple banana cherry"
for item in $items; do
    echo "$item"
done

# Line-separated (more robust)
items=$(cat <<EOF
apple
banana
cherry
EOF
)

echo "$items" | while IFS= read -r item; do
    echo "$item"
done
```

### Simple Math

```sh
# Integer arithmetic
result=$((5 + 3))
result=$((10 * 20))
result=$((100 / 3))  # Integer division
result=$((17 % 5))   # Modulo

# Increment
counter=$((counter + 1))
# or
counter=$((counter++))  # Not POSIX, avoid

# Floating point (requires bc)
result=$(echo "scale=2; 10 / 3" | bc)
```

## Using AWK Effectively

AWK is your best friend for text processing in shell scripts.

### Basic AWK Patterns

```sh
# Print specific columns
awk '{print $1, $3}' input.txt

# Filter and print
awk '$2 > 100 {print $1}' input.txt

# Sum a column
awk '{sum += $1} END {print sum}' input.txt

# Count matches
awk '/pattern/ {count++} END {print count}' input.txt
```

### AWK for Complex Processing

```sh
# Multiple conditions
awk '
    $1 == "start" { in_section = 1 }
    $1 == "end"   { in_section = 0 }
    in_section    { sum += $2 }
    END           { print sum }
' input.txt

# Arrays in AWK
awk '
    { count[$1]++ }
    END {
        for (key in count) {
            print key, count[key]
        }
    }
' input.txt
```

## Common Challenges

### Challenge: Large Numbers

Shell arithmetic is limited to integers. For large numbers:

```sh
# Use bc
result=$(echo "9999999999999 * 9999999999999" | bc)

# Or Python one-liner
result=$(python3 -c "print(9999999999999 * 9999999999999)")
```

### Challenge: Hash Maps

POSIX shell has no associative arrays. Solutions:

```sh
# Option 1: Use awk (awk has associative arrays)
result=$(awk '
    { count[$1]++ }
    END { for (k in count) print k, count[k] }
' input.txt)

# Option 2: Simulate with variables (limited)
key_value="key1:value1 key2:value2"

# Option 3: Use temporary files
echo "value1" > "/tmp/map_key1"
value=$(cat "/tmp/map_key1")
```

### Challenge: Recursion

Shell doesn't handle deep recursion well. Consider:

```sh
# Iterative approach (preferred)
fibonacci_iter() {
    n=$1
    a=0
    b=1
    i=0

    while [ $i -lt "$n" ]; do
        temp=$b
        b=$((a + b))
        a=$temp
        i=$((i + 1))
    done

    echo "$a"
}

# Recursive (avoid for deep calls)
fibonacci_rec() {
    n=$1
    if [ "$n" -le 1 ]; then
        echo "$n"
    else
        a=$(fibonacci_rec $((n - 1)))
        b=$(fibonacci_rec $((n - 2)))
        echo $((a + b))
    fi
}
```

## POSIX Compliance Tips

### Avoid Bash-isms

```sh
# ❌ Bash-specific
if [[ "$var" == "value" ]]; then
    echo "Match"
fi

# ✅ POSIX compliant
if [ "$var" = "value" ]; then
    echo "Match"
fi

# ❌ Bash arrays
declare -a array
array[0]="value"

# ✅ POSIX alternative
set -- "item1" "item2" "item3"
echo "$1"  # item1
```

### Portable String Operations

```sh
# Length
length=${#string}  # Works in POSIX

# Substring (not POSIX - use cut/sed instead)
# substring=${string:0:5}  # Bash only

# POSIX alternative
substring=$(echo "$string" | cut -c1-5)
```

### Safe Variable Handling

```sh
# Always quote variables
if [ "$var" = "value" ]; then  # ✅
if [ $var = "value" ]; then    # ❌ Can break with spaces

# Check if variable is set
if [ -n "${var+x}" ]; then
    echo "Variable is set"
fi

# Default values
value="${var:-default}"  # Use default if var is unset/empty
```

## Testing

### Simple Test Framework

```sh
#!/bin/sh
# test.sh

. ./part1.sh  # Source the solution

# Test counter
tests_passed=0
tests_failed=0

# Test function
test_case() {
    description="$1"
    expected="$2"
    input="$3"

    result=$(echo "$input" | solve -)

    if [ "$result" = "$expected" ]; then
        echo "✓ $description"
        tests_passed=$((tests_passed + 1))
    else
        echo "✗ $description"
        echo "  Expected: $expected"
        echo "  Got: $result"
        tests_failed=$((tests_failed + 1))
    fi
}

# Run tests
test_case "Example 1" "42" "example input"
test_case "Example 2" "84" "another example"

# Summary
echo ""
echo "Tests passed: $tests_passed"
echo "Tests failed: $tests_failed"

[ $tests_failed -eq 0 ] && exit 0 || exit 1
```

## Performance Tips

### Use Built-ins Over External Commands

```sh
# ❌ Slower (spawns subprocess)
length=$(echo "$string" | wc -c)

# ✅ Faster (shell built-in)
length=${#string}
```

### Minimize Subshells

```sh
# ❌ Multiple subshells
result=$(echo $(cat file | grep pattern))

# ✅ Single pipeline
result=$(grep pattern < file)
```

### Process Large Files Efficiently

```sh
# ❌ Reading entire file into memory
content=$(cat large_file.txt)
# Process $content

# ✅ Stream processing
while IFS= read -r line; do
    # Process each line
done < large_file.txt
```

## Useful One-Liners

```sh
# Find all unique values in column 1
cut -d' ' -f1 input.txt | sort -u

# Count occurrences of each value
sort input.txt | uniq -c | sort -rn

# Sum all numbers in a file
awk '{s+=$1} END {print s}' numbers.txt

# Get lines between two patterns
sed -n '/START/,/END/p' input.txt

# Replace all occurrences
sed 's/old/new/g' input.txt

# Extract email addresses
grep -o '[a-zA-Z0-9._%+-]\+@[a-zA-Z0-9.-]\+\.[a-zA-Z]\{2,\}' input.txt
```

## When NOT to Use Shell

Consider another language if you need:

- Complex data structures (trees, graphs)
- Heavy computation (factorial, large numbers)
- Deep recursion
- Advanced algorithms (Dijkstra, dynamic programming)
- Object-oriented design

Shell is a tool in your toolbox—use it for what it's good at!

## Resources

- [POSIX Shell Specification](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html)
- [ShellCheck](https://www.shellcheck.net/) - Linting tool
- [Awk Manual](https://www.gnu.org/software/gawk/manual/)
- [Sed Tutorial](https://www.grymoire.com/Unix/Sed.html)
- [Unix Text Processing](https://www.oreilly.com/library/view/classic-shell-scripting/0596005954/)

## Tips for Success

1. **Start simple** - Use shell for text processing challenges
2. **Learn awk** - It's incredibly powerful for AoC
3. **Test incrementally** - Shell scripts can be hard to debug
4. **Use ShellCheck** - Catches many common errors
5. **Know when to switch** - Some puzzles are better suited for other languages
6. **Embrace pipelines** - Chain commands together
7. **Read the man pages** - `man awk`, `man sed`, etc.

---
