#!/bin/bash
mkdir -p ./target/release
DUMMYS=( "dummy" "dummy2" "dummy3" "how_many_fds" "print_args" "sleeper" "signals" "sigtrap_self" "ptrace_self")

mkdir -p ./target/debug/
for DUMMY in ${DUMMYS[*]}; do
	BINPATH=./target/debug/${DUMMY}
	gcc -g ./examples/${DUMMY}.c --output $BINPATH
	echo $BINPATH
done
