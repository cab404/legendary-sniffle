#!/bin/sh
set -e
cp ../examples/future-generations.md text.md
cp ../examples/future-generations.json text.json


runCycle() {
    time cargo run --release -- --old-json text.json --new-string text.md --new-json text.json --new-keys keys.json
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
