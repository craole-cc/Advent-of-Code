#!/bin/sh

#> Try git command (git checked in dependencies)
git_root=$(git rev-parse --show-toplevel 2>/dev/null)
if [ -n "${git_root}" ]; then
	printf "%s" "${git_root}"
	exit 0
fi

#> Walk up looking for .git (if not in repo or detached)
script_dir="$(cd "$(dirname "$0")" && pwd)"
current_dir="${script_dir}"

while [ "${current_dir}" != "/" ]; do
	if [ -d "${current_dir}/.git" ]; then
		printf "%s" "${current_dir}"
		exit 0
	fi
	current_dir="$(dirname "${current_dir}")"
done

#> Fallback to the parent of the script directory
printf "%s" "$(dirname "${script_dir}")"
