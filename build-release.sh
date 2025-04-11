#!/bin/bash
mkdir -p ./target/release
DUMMYS=( "dummy" "dummy2" "dummy3" "how_many_fds" "print_args" "sleeper" "signals" "sigtrap_self" "ptrace_self")

mkdir -p ./target/release/
for DUMMY in ${DUMMYS[*]}; do
	BINPATH=./target/release/${DUMMY}
	gcc ./examples/${DUMMY}.c --output $BINPATH
	strip $BINPATH
	echo $BINPATH
done
