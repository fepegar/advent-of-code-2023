#!/bin/bash

this_dir=$(realpath $(dirname $0))
for day_dir in $(ls -d src/day_*/); do
    echo "RUNNING $day_dir"
    cd $day_dir
    cargo run -q
    cd $this_dir
    echo
    echo "===================="
    echo
done
