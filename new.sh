#!/usr/bin/env bash
# Creates a new Rust project
set -e

day_of_the_month=$(date +%d | sed 's/^0*//')
day_of_the_month=$((day_of_the_month + 1))
if [[ $day_of_the_month -lt 10 ]]; then
  day_of_the_month="0${day_of_the_month}"
fi
name="day${day_of_the_month}"

if [[ -e "$name" ]]; then
  echo "Error: $name already exists." >&2
  exit 1
fi

echo "$(tput bold)Creating a new project for day ${day_of_the_month}...$(tput sgr0)"

# Create a new Cargo project
cargo new --bin "$name"
rm -f "$name/Cargo.toml" "$name/Cargo.lock"

# Add to the Cargo.toml
cat >>Cargo.toml <<EOF

[[bin]]
name = "$name"
path = "$name/main.rs"
EOF

echo "$(tput bold)Done!$(tput sgr0)"
exec code .
