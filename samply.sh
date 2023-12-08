#!/bin/bash
set -e

cargo build --profile profiling && clear &&
	RUST_LOG=info samply record target/profiling/aoc2023 --force-all --n 100000
