#!/usr/bin/env bash
# Creates a new Rust project
set -e

# year is the current year.
year=$(date -u +%Y)

# op_entry_id is the ID of the 1password entry containing the AoC
# session cookie. Should be in the field "session_cookie".
op_entry_id="mrul5kpl2iifie7v2y7h7pm43q"

# day_of_the_month is the numeric day of the month used for setting up
# an advent of code folder. This will always be the current date +1 with
# the assumption being that you are creating the project in Pacific
# Time, which would be behind at the time of creation.
day_of_the_month=$(date +%d | sed 's/^0*//')

# day_of_the_month_fmt is the formatted day of the month used for the
# project directory.
day_of_the_month_fmt="${day_of_the_month}"

# We use the 1Password CLI to store the AoC session cookie.
if ! op --version >/dev/null 2>&1; then
  echo "Error: op is not installed." >&2
  exit 1
fi

# Ensure we're logged in.
if ! op whoami >/dev/null 2>&1; then
  echo "$(tput bold)Logging in to 1Password...$(tput sgr0)" >&2
  eval "$(op signin)"
fi

# Normalize the day_of_the_month value.
day_of_the_month=$((day_of_the_month + 1))
if [[ "$1" == "--previous" ]]; then
  day_of_the_month=$((day_of_the_month - 1))
fi

# If we're below 10, add a leading zero for consistency.
if [[ $day_of_the_month -lt 10 ]]; then
  day_of_the_month_fmt="0${day_of_the_month}"
fi

# name is the name of the project. Corresponds to the day of the month
# and the project directory name.
name="day${day_of_the_month_fmt}"
if [[ -e "$name" ]]; then
  echo "Error: $name already exists." >&2
  exit 1
fi

echo "$(tput bold)Creating a new project for day ${day_of_the_month_fmt}...$(tput sgr0)"

# Create a new Cargo project
cargo new --bin "$name"
rm -f "$name/Cargo.toml" "$name/Cargo.lock"

# Download the input
echo "$(tput bold)Downloading input...$(tput sgr0)"
wget -O "$name/input.txt" "https://adventofcode.com/${year}/day/${day_of_the_month}/input" \
  --header="Cookie: session=$(op item get "$op_entry_id" --fields "session_cookie")"

# Add to the Cargo.toml
cat >>Cargo.toml <<EOF

[[bin]]
name = "$name"
path = "$name/main.rs"
EOF

echo "$(tput bold)Done!$(tput sgr0)"
#exec code .
