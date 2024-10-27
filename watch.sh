#!/bin/bash
set -e

# RUST_LOG=trace cargo watch -c -w src -x 'run -- --force-all --validate'
RUST_LOG=trace cargo watch -c -w src -x 'build --profile release && clear && ./target/release/aoc2023 --force-all --validate'
