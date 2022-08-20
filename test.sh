#!/bin/sh
set -e

cp ./examples/future-generations.md text.md

cp ./examples/future-generations.md text.md
cp ./examples/future-generations.json text.json

runCycle() {
    cargo run -- text.json text.md && mv new-text.json text.json
}

{ echo $'test\n\n'; cat text.md; } > text2.md; mv text2.md text.md
runCycle

{ echo $'test\n\n'; cat text.md; } > text2.md; mv text2.md text.md
runCycle

{ echo $'test\n\n'; cat text.md; } > text2.md; mv text2.md text.md
runCycle
