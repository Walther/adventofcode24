# List the available recipes
default:
  @just --list --unsorted

# Run a specific day
@day DAY:
  echo "Advent of Code Day {{DAY}}"
  cargo run --quiet --release --bin day-$(printf "%02d" {{DAY}})

# Run all tests
@test:
  cargo nextest run --cargo-quiet
