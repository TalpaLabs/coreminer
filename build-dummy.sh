#!/bin/bash
mkdir -p ./target/debug
gcc -g ./examples/dummy.c --output ./target/debug/dummy
gcc -g ./examples/dummy2.c --output ./target/debug/dummy2
gcc -g ./examples/dummy3.c --output ./target/debug/dummy3
