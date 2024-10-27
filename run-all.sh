#!/bin/bash
set -e

cargo build --profile release && clear &&
  RUST_LOG=trace ./target/release/aoc2023 --force-all --validate
