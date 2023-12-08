#!/bin/bash
set -e

cargo build --profile release && clear &&
	RUST_LOG=debug ./target/release/aoc2023 --force-all --validate
