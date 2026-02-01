#!/bin/sh
# shellcheck enable=all
# POSIX-compliant script to fetch Advent of Code assets
# Usage: ./fetch-aoc.sh [<year> <day> | <day> | all | latest]
# With no arguments: interactive mode

set -e

#~@ ------------------------------------------------------------
#~@ Configuration                                              .
#~@ ------------------------------------------------------------

SCRIPT_NAME="$(basename "$0")"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PRJ_ROOT="${PRJ_ROOT:-$("${SCRIPT_DIR}/locate-root.sh")}"
DATA_DIR="${PRJ_ROOT}/assets/data"
SESSION_KEY_FILE="${PRJ_ROOT}/.env/session.key"
AOC_BASE_URL="https://adventofcode.com"
FIRST_YEAR=2015
msg=""

#~@ ------------------------------------------------------------
#~@ Utilities                                                  .
#~@ ------------------------------------------------------------

print_usage() {
	cat <<-EOF
		Usage: ${SCRIPT_NAME} [<year> <day> | <day> | all | latest]

		With arguments:
			all               Fetch all missing data from 2015 to current
			latest            Fetch the most recent puzzle
			<day>             Fetch specified day from the most recent year
			<year> <day>      Fetch specific year and day

		No arguments:       Interactive mode

		Examples:
			${SCRIPT_NAME}              # Interactive mode
			${SCRIPT_NAME} latest       # Fetch today's/latest puzzle
			${SCRIPT_NAME} all          # Fetch all missing data
			${SCRIPT_NAME} 5            # Fetch day 5 of latest year
			${SCRIPT_NAME} 2023 5       # Fetch 2023 day 5
	EOF
	exit 1
}

error_exit() {
	printf "Error: %s\n" "$*" >&2
	exit 1
}

log_info() {
	printf "%s\n" "$*"
}

is_integer() {
	case "${1#[+-]}" in
	'' | *[!0-9]*) return 1 ;;
	*) return 0 ;;
	esac
}

#~@ ------------------------------------------------------------
#~@ Dependency and Setup Checks                                .
#~@ ------------------------------------------------------------

check_dependencies() {
	for cmd in git curl; do
		if ! command -v "${cmd}" >/dev/null 2>&1; then
			error_exit "${cmd} is required but not installed"
		fi
	done

	log_info "Dependencies validated"
}

read_session_key() {
	if [ -n "${SESSION_KEY:-}" ]; then
		#> Attempt to read the session key from the variable
		session_key=$(printf "%s" "${SESSION_KEY}")
	elif [ -f "${SESSION_KEY_FILE}" ]; then
		#> Attempt to read the session key from the file
		session_key=$(cat "${SESSION_KEY_FILE}")
	else
		error_exit "$(
			printf "Session key not found. Please either:"
			printf "\n 1. Set the SESSION_KEY environment variable, OR"
			printf "\n 2. Place the key in a session.key file at: %s\n" "${SESSION_KEY_FILE}"
		)"
	fi

	#> Return the session key
	printf "%s" "${session_key:-}" | tr -d '[:space:]'
}

#~@ ------------------------------------------------------------
#~@ Date Validation                                            .
#~@ ------------------------------------------------------------

calculate_est_time() {
	#> Get current UTC time and convert to EST (UTC-5)
	current_year=$(date -u +%Y)
	current_month=$(date -u +%m)
	current_day=$(date -u +%d)
	current_hour=$(date -u +%H)

	#> Calculate EST time (UTC-5)
	est_hour=$((current_hour - 5))
	if [ "${est_hour}" -lt 0 ]; then
		est_hour=$((est_hour + 24))
		est_day=$((current_day - 1))
		est_month="${current_month}"
		est_year="${current_year}"

		#> Handle month/year rollover
		if [ "${est_day}" -eq 0 ]; then
			case "${current_month}" in
			1)
				est_day=31
				est_month=12
				est_year=$((current_year - 1))
				;;
			2 | 4 | 6 | 8 | 9 | 11)
				est_day=31
				est_month=$((current_month - 1))
				;;
			3)
				est_day=$((current_year % 4 == 0 && (current_year % 100 != 0 || current_year % 400 == 0) ? 29 : 28))
				est_month=2
				;;
			5 | 7 | 10 | 12)
				est_day=30
				est_month=$((current_month - 1))
				;;
			*)
				# TODO: Add a fallback/error handling
				;;
			esac
		fi
	else
		est_day="${current_day}"
		est_month="${current_month}"
		est_year="${current_year}"
	fi

	printf "%d %d %d %d\n" "${est_year}" "${est_month}" "${est_day}" "${est_hour}"
}

get_max_allowed_date() {
	est_info=$(calculate_est_time)
	est_year=$(printf '%s' "${est_info}" | cut -d' ' -f1)
	est_month=$(printf '%s' "${est_info}" | cut -d' ' -f2)
	est_day=$(printf '%s' "${est_info}" | cut -d' ' -f3)
	current_hour=$(date -u +%H)
	current_day="$(date -u +%d)"

	#? Default values
	max_year="${est_year}"
	max_day=25

	#? If we're before December, max year is previous year
	if [ "${est_month}" -lt 12 ]; then
		max_year=$((est_year - 1))
	else
		#? During Advent period (Dec 1-25)
		max_day="${est_day}"

		#> Check if before release time (05:00 UTC)
		if [ "${current_hour}" -lt 5 ] && [ "${est_day}" -eq "${current_day}" ]; then
			max_day=$((est_day - 1))
			if [ "${max_day}" -eq 0 ]; then
				max_year=$((est_year - 1))
				max_day=25
			fi
		fi
	fi

	#> Ensure min values
	[ "${max_day}" -lt 1 ] && max_day=1

	printf "%d %d\n" "${max_year}" "${max_day}"
}

validate_date() {
	year="$1"
	day="$2"
	max_year="$3"
	max_day="$4"

	#> Validate year
	if [ "${year}" -lt "${FIRST_YEAR}" ] || [ "${year}" -gt "${max_year}" ]; then
		if [ "${max_year}" -lt "${FIRST_YEAR}" ]; then
			msg="$(printf "Advent of Code was launched in %s." "${FIRST_YEAR}")"
			error_exit "${msg}"
		fi
		msg="$(
			printf "Year must be between %s and %s (inclusive)" \
				"${FIRST_YEAR}" \
				"${max_year}"
		)"
		error_exit "${msg}"
	fi

	#> Validate day
	if [ "${year}" -eq "${max_year}" ]; then
		#? Current year
		if [ "${day}" -lt 1 ] || [ "${day}" -gt "${max_day}" ]; then
			current_est=$(
				date -d 'TZ="EST" +%Y-%m-%d\ %H:%M' 2>/dev/null ||
					date -u +"%Y-%m-%d %H:%M UTC"
			)
			if [ "${day}" -gt "${max_day}" ]; then
				msg="$(
					printf \
						"Day %s hasn't been released yet! Latest available: day %s (Current EST: %s)" \
						"${day}" "${max_day}" "${current_est}"
				)"
				error_exit "${msg}"
			fi
			msg="$(printf "For %s, day must be between 1 and %s" "${year}" "${max_day}")"
			error_exit "${msg}"
		fi
	else
		#? Past years
		if [ "${day}" -lt 1 ] || [ "${day}" -gt 25 ]; then
			error_exit "Day must be between 1 and 25"
		fi
	fi

	#> Check release timing for current day
	if [ "${year}" -eq "${max_year}" ] && [ "${day}" -eq "${max_day}" ]; then
		check_release_timing
	fi
}

check_release_timing() {
	current_hour_utc=$(date -u +%H)
	if [ "${current_hour_utc}" -lt 5 ]; then
		hours_until=$((5 - current_hour_utc))
		minutes_until=$((60 - $(date -u +%M)))
		[ "${minutes_until}" -eq 60 ] && minutes_until=0

		msg="$(
			printf "Today's puzzle hasn't been released yet! "
			printf "It releases in ~%dh %dm (05:00 UTC / 00:00 EST)" \
				"${hours_until}" "${minutes_until}"
		)"
		error_exit "${msg}"
	fi
}

#~@ ------------------------------------------------------------
#~@ Interactive Mode                                           .
#~@ ------------------------------------------------------------

show_menu() {
	printf '\n╔════════════════════════════════════════════╗'
	printf '\n║     Advent of Code Asset Fetcher           ║'
	printf '\n╚════════════════════════════════════════════╝'
	printf '\n\nWhat would you like to fetch?'
	printf '\n\n  1) Latest puzzle (currently: %d day %d)' "${max_year}" "${max_day}"
	printf '\n\n  2) Specific day of current year (%d)' "${max_year}"
	printf '\n\n  3) Specific year and day'
	printf '\n\n  4) All missing data (%d-%d)' "${FIRST_YEAR}" "${max_year}"
	printf '\n\n  5) Show help'
	printf '\n\n  6) Exit'
	printf '\n\n  Enter choice [1-6]: '
}

run_interactive_mode() {
	max_allowed=$(get_max_allowed_date)
	max_year=$(printf '%s' "${max_allowed}" | cut -d' ' -f1)
	max_day=$(printf '%s' "${max_allowed}" | cut -d' ' -f2)

	show_menu

	while true; do
		read -r choice

		case "${choice}" in
		1)
			printf '\n'
			log_info "Fetching latest puzzle: ${max_year} day ${max_day}"
			validate_date "${max_year}" "${max_day}" "${max_year}" "${max_day}"
			# Set variables directly instead of printing
			MODE="single"
			YEAR="${max_year}"
			DAY="${max_day}"
			return
			;;
		2)
			printf '\nEnter day (1-%d): ' "${max_day}"
			read -r day
			# shellcheck disable=SC2310
			if ! is_integer "${day}" || [ "${day}" -lt 1 ] || [ "${day}" -gt "${max_day}" ]; then
				printf "Invalid day. Must be a number between 1 and %d.\n\n" "${max_day}"
				show_menu
				continue
			fi
			printf '\n'
			log_info "Fetching ${max_year} day ${day}"
			validate_date "${max_year}" "${day}" "${max_year}" "${max_day}"
			# Set variables directly instead of printing
			MODE="single"
			YEAR="${max_year}"
			DAY="${day}"
			return
			;;
		3)
			printf '\nEnter year (%d-%d): ' "${FIRST_YEAR}" "${max_year}"
			read -r year
			# shellcheck disable=SC2310
			if ! is_integer "${year}" ||
				[ "${year}" -lt "${FIRST_YEAR}" ] ||
				[ "${year}" -gt "${max_year}" ]; then
				printf "Invalid year. Must be a number between %d and %d.\n\n" \
					"${FIRST_YEAR}" "${max_year}"
				show_menu
				continue
			fi
			year_max_day=$(
				if [ "${year}" = "${max_year}" ]; then
					echo "${max_day}"
				else
					echo 25
				fi
			)
			printf 'Enter day (1-%d): ' "${year_max_day}"
			read -r day
			if
				# shellcheck disable=SC2310
				! is_integer "${day}" ||
					[ "${day}" -lt 1 ] ||
					[ "${day}" -gt "${year_max_day}" ]
			then
				printf \
					"Invalid day. Must be a number between 1 and %d for year %d.\n\n" \
					"${year_max_day}" "${year}"
				show_menu
				continue
			fi
			printf '\n'
			log_info "Fetching ${year} day ${day}"
			validate_date "${year}" "${day}" "${max_year}" "${max_day}"
			# Set variables directly instead of printing
			MODE="single"
			YEAR="${year}"
			DAY="${day}"
			return
			;;
		4)
			printf '\nThis will fetch all missing puzzles from %d to %d.\n' "${FIRST_YEAR}" "${max_year}"
			printf 'It may take several minutes and make many requests.\n'
			printf 'Continue? [y/N]: '
			read -r confirm
			case "${confirm}" in [yY] | [yY][eE][sS])
				printf '\n'
				log_info "Fetching all missing data from ${FIRST_YEAR} to ${max_year}"
				# Set variables directly instead of printing
				MODE="all"
				YEAR="${max_year}"
				DAY="${max_day}"
				return
				;;
			*)
				printf "Cancelled.\n\n"
				show_menu
				continue
				;;
			esac
			;;
		5) print_usage ;;
		6)
			printf '\nGoodbye!\n'
			exit 0
			;;
		*)
			printf "Invalid choice. Please enter 1-6.\n\n"
			show_menu
			continue
			;;
		esac
	done
}

#~@ ------------------------------------------------------------
#~@ Argument Parsing
#~@ ------------------------------------------------------------

parse_arguments() {
	max_allowed=$(get_max_allowed_date)
	max_year=$(printf '%s' "${max_allowed}" | cut -d' ' -f1)
	max_day=$(printf '%s' "${max_allowed}" | cut -d' ' -f2)

	case $# in
	0)
		run_interactive_mode
		# MODE, YEAR, and DAY are now set by run_interactive_mode
		;;
	1)
		case "$1" in
		"all" | "missing" | "gap")
			MODE="all"
			YEAR="${max_year}"
			DAY="${max_day}"
			;;
		"latest" | "today" | "current")
			validate_date "${max_year}" "${max_day}" "${max_year}" "${max_day}"
			MODE="single"
			YEAR="${max_year}"
			DAY="${max_day}"
			;;
		*)
			# shellcheck disable=SC2310
			if is_integer "$1" && [ "$1" -ge 1 ] && [ "$1" -le "${max_day}" ]; then
				validate_date "${max_year}" "$1" "${max_year}" "${max_day}"
				MODE="single"
				YEAR="${max_year}"
				DAY="$1"
			else
				error_exit "Invalid argument: '$1'. Use: <day>, 'all', or 'latest'"
			fi
			;;
		esac
		;;
	2)
		# shellcheck disable=SC2310
		if ! is_integer "$1" || ! is_integer "$2"; then
			error_exit "Both arguments must be numbers"
		fi
		validate_date "$1" "$2" "${max_year}" "${max_day}"
		MODE="single"
		YEAR="$1"
		DAY="$2"
		;;
	*) print_usage ;;
	esac

	log_info "Initializing Environment"
	log_info "MODE:" "${MODE}"
	log_info "YEAR:" "${YEAR}"
	log_info "DAY:" "${DAY}"
}

#~@ ------------------------------------------------------------
#~@ File System Operations
#~@ ------------------------------------------------------------

create_day_directory() {
	year="$1"
	day="$2"

	day_padded=$(printf "%02d" "${day}")
	day_dir="${DATA_DIR}/${year}/${day_padded}"

	if [ ! -d "${day_dir}" ]; then
		mkdir -p "${day_dir}"
		log_info "Created directory: ${day_dir}" >&2
	fi

	printf "%s" "${day_dir}"
}

create_output_placeholder() {
	output_file="$1"

	if [ ! -f "${output_file}" ]; then
		printf "# Output for this puzzle will go here\n" >"${output_file}"
		log_info "Created output placeholder: ${output_file}" >&2
	fi
}

#~@ ------------------------------------------------------------
#~@ Fetch Operations
#~@ ------------------------------------------------------------

fetch_with_curl() {
	url="$1"
	session_key="$2"
	output_file="$3"
	max_retries="${4:-3}" # Default to 3 retries if not specified

	log_info "Fetching from: ${url}"

	retry_count=0

	while [ "${retry_count}" -le "${max_retries}" ]; do
		# When using -o, curl's -w output goes to stdout (not the file)
		# So we can capture it directly in a variable
		http_code=$(curl -s -o "${output_file}" -w '%{http_code}' -b "session=${session_key}" "${url}" 2>/dev/null)

		# If curl failed entirely, default to 000
		[ -z "${http_code}" ] && http_code="000"

		case "${http_code}" in
		200)
			# Verify we actually got content
			if [ -s "${output_file}" ]; then
				log_info "Successfully saved to: ${output_file}"
				return 0
			else
				# Empty response with HTTP 200 is a sign of rate limiting
				# Advent of Code does this instead of returning 429
				retry_count=$((retry_count + 1))
				if [ "${retry_count}" -le "${max_retries}" ]; then
					# Exponential backoff for empty responses
					delay=1
					i=0
					while [ "${i}" -lt "${retry_count}" ]; do
						delay=$((delay * 2))
						i=$((i + 1))
					done
					printf "HTTP 200 but empty response (likely rate limited). Retry %d/%d in %d seconds...\n" \
						"${retry_count}" "${max_retries}" "${delay}" >&2
					sleep "${delay}"
				else
					printf "HTTP 200 but empty response. Max retries exceeded (likely rate limited)\n" >&2
					return 1
				fi
			fi
			;;
		400)
			# Bad request - puzzle doesn't exist yet or invalid request
			printf "HTTP 400: Puzzle may not be released yet or invalid request\n" >&2
			return 1
			;;
		404)
			# Not found - puzzle doesn't exist
			printf "HTTP 404: Puzzle not found (day may not exist)\n" >&2
			return 1
			;;
		429)
			# Rate limited - retry with exponential backoff
			retry_count=$((retry_count + 1))
			if [ "${retry_count}" -le "${max_retries}" ]; then
				# Exponential backoff: 2^retry * base_delay (POSIX-compliant)
				delay=1
				i=0
				while [ "${i}" -lt "${retry_count}" ]; do
					delay=$((delay * 2))
					i=$((i + 1))
				done
				printf "HTTP 429: Rate limited. Retry %d/%d in %d seconds...\n" \
					"${retry_count}" "${max_retries}" "${delay}" >&2
				sleep "${delay}"
			else
				printf "HTTP 429: Rate limited. Max retries exceeded.\n" >&2
				return 1
			fi
			;;
		500 | 502 | 503 | 504)
			# Server errors - retry with backoff
			retry_count=$((retry_count + 1))
			if [ "${retry_count}" -le "${max_retries}" ]; then
				# Exponential backoff: 2^retry (POSIX-compliant)
				delay=1
				i=0
				while [ "${i}" -lt "${retry_count}" ]; do
					delay=$((delay * 2))
					i=$((i + 1))
				done
				printf "HTTP %s: Server error. Retry %d/%d in %d seconds...\n" \
					"${http_code}" "${retry_count}" "${max_retries}" "${delay}" >&2
				sleep "${delay}"
			else
				printf "HTTP %s: Server error. Max retries exceeded.\n" "${http_code}" >&2
				return 1
			fi
			;;
		401 | 403)
			# Authentication errors - don't retry
			printf "HTTP %s: Invalid or expired session key\n" "${http_code}" >&2
			return 1
			;;
		000)
			# Connection error (curl couldn't connect)
			printf "Connection failed: Network error or server unreachable\n" >&2
			return 1
			;;
		*)
			# Unknown error - show what we got and the first line of response
			printf "HTTP %s: Unexpected error\n" "${http_code}" >&2
			if [ -f "${output_file}" ] && [ -s "${output_file}" ]; then
				printf "Response preview: %s\n" "$(head -1 "${output_file}")" >&2
			fi
			return 1
			;;
		esac
	done

	return 1
}

fetch_input() {
	year="$1"
	day="$2"
	output_file="$3"
	session_key="$4"

	url="${AOC_BASE_URL}/${year}/day/${day}/input"

	# shellcheck disable=SC2310
	if ! fetch_with_curl "${url}" "${session_key}" "${output_file}"; then
		return 1
	fi
}

fetch_instructions() {
	year="$1"
	day="$2"
	output_file="$3"
	session_key="$4"

	url="${AOC_BASE_URL}/${year}/day/${day}"
	html=$(curl -f -s -b "session=${session_key}" "${url}")

	if [ -z "${html}" ]; then
		error_exit "Failed to fetch instructions"
	fi

	#> Extract content between <article> tags
	printf "%s\n" "${html}" |
		sed -n '/<article class="day-desc">/,/<\/article>/p' >"${output_file}"

	if [ -s "${output_file}" ]; then
		log_info "Successfully saved instructions to: ${output_file}"
	else
		error_exit "Failed to extract instructions from HTML"
	fi
}

#~@ ------------------------------------------------------------
#~@ Data Fetching
#~@ ------------------------------------------------------------

fetch_single_data() {
	year="$1"
	day="$2"
	session_key="$3"

	day_dir=$(create_day_directory "${year}" "${day}")

	#> Define file paths
	input_file="${day_dir}/input.txt"
	instructions_file="${day_dir}/instructions.txt"
	output_file="${day_dir}/output.txt"

	#> Fetch assets - track failures but don't exit immediately
	fetch_failed=0

	# shellcheck disable=SC2310
	if ! fetch_input "${year}" "${day}" "${input_file}" "${session_key}"; then
		fetch_failed=1
	fi

	# shellcheck disable=SC2310
	if ! fetch_instructions "${year}" "${day}" "${instructions_file}" "${session_key}"; then
		fetch_failed=1
	fi

	create_output_placeholder "${output_file}"

	if [ "${fetch_failed}" -eq 1 ]; then
		return 1
	fi

	printf "%s" "${day_dir}"
}

fetch_all_missing_data() {
	max_year="$1"
	max_day="$2"
	session_key="$3"

	total_years=$((max_year - FIRST_YEAR + 1))
	total_puzzles=0

	#? Calculate total puzzles
	for year in $(seq "${FIRST_YEAR}" "${max_year}"); do
		if [ "${year}" -eq "${max_year}" ]; then
			total_puzzles=$((total_puzzles + max_day))
		else
			total_puzzles=$((total_puzzles + 25))
		fi
	done

	log_info "Fetching all missing data from ${FIRST_YEAR} to ${max_year}"
	log_info "Total puzzles to check: ${total_puzzles}"
	log_info "Using 2-5 second delays between requests to respect server limits"
	log_info ""

	fetched_count=0
	skipped_count=0
	error_count=0
	rate_limited_count=0

	for year in $(seq "${FIRST_YEAR}" "${max_year}"); do
		#> Determine days to check for this year
		if [ "${year}" -eq "${max_year}" ]; then
			end_day="${max_day}"
		else
			end_day=25
		fi

		log_info "Checking year ${year} (days 1-${end_day})..."

		for day in $(seq 1 "${end_day}"); do
			day_padded=$(printf "%02d" "${day}")
			day_dir="${DATA_DIR}/${year}/${day_padded}"
			input_file="${day_dir}/input.txt"

			if [ -f "${input_file}" ] && [ -s "${input_file}" ]; then
				skipped_count=$((skipped_count + 1))
				printf "  Day %02d: Already exists, skipping\n" "${day}"
			else
				printf "  Day %02d: Fetching...\n" "${day}"
				# shellcheck disable=SC2310
				if fetch_output=$(fetch_single_data "${year}" "${day}" "${session_key}" 2>&1); then
					fetched_count=$((fetched_count + 1))
					printf "\t✓ Success\n"
				else
					error_count=$((error_count + 1))
					# Check if error was due to rate limiting
					if echo "${fetch_output}" | grep -qE "(429|empty response)"; then
						rate_limited_count=$((rate_limited_count + 1))
						printf "\t⏸ Rate limited - will retry later\n"
					else
						printf "\t✗ Failed\n" >&2
						# Only show detailed error for non-rate-limit errors
						if [ -n "${fetch_output}" ]; then
							printf "\t  %s\n" "${fetch_output}" | sed "s/^/\t  /" >&2
						fi
					fi
				fi

				#? Randomized delay (2-5 seconds) to appear more human and avoid rate limiting
				# Using modulo of seconds since epoch for pseudo-randomness
				random_component=$(($(date +%s) % 4))
				sleep_time=$((2 + random_component))
				sleep "${sleep_time}"
			fi
		done

		log_info ""
	done

	#? Print summary
	log_info "========================================"
	log_info "Summary:"
	log_info "  Years checked:      ${total_years} (${FIRST_YEAR}-${max_year})"
	log_info "  Puzzles fetched:    ${fetched_count}"
	log_info "  Puzzles skipped:    ${skipped_count}"
	log_info "  Errors encountered: ${error_count}"
	if [ "${rate_limited_count}" -gt 0 ]; then
		log_info "  Rate limited:       ${rate_limited_count} (increase delays and retry)"
	fi
	log_info "  Total puzzles:      ${total_puzzles}"
	log_info ""

	if [ "${fetched_count}" -gt 0 ]; then
		log_info "✓ All missing data fetched successfully!"
	elif [ "${error_count}" -eq 0 ]; then
		log_info "✓ All puzzles are already up to date!"
	else
		log_info "⚠ Some errors occurred. Check the output above."
		if [ "${rate_limited_count}" -gt 0 ]; then
			log_info "💡 Tip: If rate limited, wait a few minutes and re-run to fetch remaining puzzles."
		fi
	fi
}

#~@ ------------------------------------------------------------
#~@ Script Entry Point
#~@ ------------------------------------------------------------

main() {
	#> Check dependencies first
	check_dependencies

	#> Initialize the environment
	parse_arguments "$@"

	#> Read session key
	session_key=$(read_session_key)

	case "${MODE}" in
	"all")
		fetch_all_missing_data "${YEAR}" "${DAY}" "${session_key}"
		;;
	"single")
		# shellcheck disable=SC2310
		if fetched="$(fetch_single_data "${YEAR}" "${DAY}" "${session_key}" 2>&1)"; then
			printf "\n✓ All assets fetched successfully for %d day %d\n" "${YEAR}" "${DAY}"
			printf "Files saved in: %s\n" "${fetched}"
		else
			printf "\n✗ Failed to fetch puzzle for %d day %d\n" "${YEAR}" "${DAY}" >&2
			printf "Error details above. Common issues:\n" >&2
			printf "  - Invalid or expired session key (check .env/session.key)\n" >&2
			printf "  - Puzzle not released yet (releases at 05:00 UTC / 00:00 EST)\n" >&2
			printf "  - Network connectivity issues\n" >&2
			printf "  - Rate limiting (wait a few minutes and try again)\n" >&2
			exit 1
		fi
		;;
	*)
		error_exit "Invalid mode: ${MODE}"
		;;
	esac
}

main "$@"
