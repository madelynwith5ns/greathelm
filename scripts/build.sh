#!/usr/bin/bash
echo This Greathelm project is a thin wrapper around the Cargo project.
echo This will not build unless you have cargo installed.
echo This will eventually use a Rust builder instead of a Custom builder simply calling cargo.

cargo build
