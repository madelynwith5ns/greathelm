#!/usr/bin/bash
echo Invoking cargo...

cargo build --release
cp target/release/greathelm build/$1
