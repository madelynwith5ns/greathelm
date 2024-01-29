#!/usr/bin/bash

rm Plugins.ghm || true
echo Adding GHP-Rust
echo '@Module ghprust build/ghprust.ghp:build/ghprust.ghp' >> Plugins.ghm

echo Done! Plugins will be built on your next build.
