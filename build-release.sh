#!/bin/bash
mkdir -p ./target/release
D1=./target/release/dummy
D2=./target/release/dummy2
D3=./target/release/dummy3
gcc ./examples/dummy.c --output $D1
gcc ./examples/dummy2.c --output $D2
gcc ./examples/dummy3.c --output $D3
strip $D1
strip $D2
strip $D3
