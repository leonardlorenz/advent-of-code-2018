#!/bin/bash
# ./solution.sh /path/to/input/location
cargo build
cat $1 | tr '\n' ',' | cargo run
