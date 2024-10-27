#!/bin/bash
set -e

RUST_LOG=trace cargo watch -c -w src -x 'run -- --show-solutions --validate'
