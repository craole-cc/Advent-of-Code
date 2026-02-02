#!/bin/sh
# Rename all instructions.txt files to instructions.html

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PRJ_ROOT="${PRJ_ROOT:-$("${SCRIPT_DIR}/locate-root.sh")}"
DATA_DIR="${PRJ_ROOT}/assets/data"

printf 'Renaming all instructions.txt to instructions.html...\n'
printf 'Data directory: %s\n\n' "${DATA_DIR}"

count=0
tmpfiles=$(mktemp)
trap 'rm -f "$tmpfiles"' EXIT
find "${DATA_DIR}" -type f -name 'instructions.txt' >"$tmpfiles"

while IFS= read -r file; do
	[ -n "$file" ] || continue
	dir=$(dirname "${file}")
	short_dir=$(basename "${dir}")
	year_month_dir=$(basename "$(dirname "${dir}")")/${short_dir}
	new_file="${dir}/instructions.html"
	mv "${file}" "${new_file}"
	printf '%s done\n' "${year_month_dir}"
	count=$((count + 1))
done <"$tmpfiles"

printf '\nDone! Renamed %d files.\n' "${count}"
