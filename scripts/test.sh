#!/bin/sh
set -e
cp ../examples/future-generations.md text.md
cp ../examples/future-generations.json text.json

cargo build --release

runCycle() {
    time ../target/release/stroki text.json text.md && mv new-text.json text.json
}

for I in {1..10}; do
  insert_loc=5
  echo Cycle $I, inserting at $insert_loc
  { head -n $insert_loc text.md; echo $'\n\ntest\n\n'; tail -n +$insert_loc text.md; } > text2.md; mv text2.md text.md
  runCycle
done

for I in {1..100}; do
  insert_loc=100
  echo Cycle $I, inserting at $insert_loc
  { head -n $insert_loc text.md; echo $'\n\ntest\n\n'; tail -n +$insert_loc text.md; } > text2.md; mv text2.md text.md
  runCycle
done
