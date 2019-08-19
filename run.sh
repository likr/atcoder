#!/bin/bash
rustc $1 -O -o a.out && RUST_BACKTRACE=1 ./a.out
