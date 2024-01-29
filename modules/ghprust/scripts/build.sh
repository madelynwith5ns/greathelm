#!/usr/bin/bash

echo Invoking cargo..
cargo build --release

cp target/release/libghprust.so build/ghprust.ghp
