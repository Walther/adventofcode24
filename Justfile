# List the available recipes
default:
  @just --list --unsorted

# Run all days
@all:
  cargo build --quiet --release
  for day in `seq 1 $(fd -td "day*" | wc -l)`; do just day $day; done;

# Run a specific day
@day DAY:
  echo "Advent of Code Day {{DAY}}"
  cargo run --quiet --release --bin day-$(printf "%02d" {{DAY}})
alias d := day

# Run all tests
@test:
  cargo nextest run --cargo-quiet
alias t := test
