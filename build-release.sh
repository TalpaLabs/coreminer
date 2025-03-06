#!/bin/bash
mkdir -p ./target/release
DUMMYS=( "dummy" "dummy2" "dummy3" "how_many_fds" )

mkdir -p ./target/release/
for DUMMY in ${DUMMYS[*]}; do
	BINPATH=./target/release/${DUMMY}
	gcc ./examples/${DUMMY}.c --output $BINPATH
	strip $BINPATH
	echo $BINPATH
done
