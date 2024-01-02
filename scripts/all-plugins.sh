#!/usr/bin/bash

echo Fetching and building all plugins...

if [ -d 'modules' ]; then
    echo Modules directory exists already. Removing and starting over.
    rm -rf modules
fi

if [ -f 'Plugins.ghm' ]; then
    rm Plugins.ghm
fi

mkdir modules

echo Fetching: GHP-Rust
git clone https://github.com/Greathelm/GHP-Rust modules/ghprust
echo '@Module ghprust build/ghprust.ghp:build/ghprust.ghp' >> Plugins.ghm

echo Done! Plugins will be built on your next build.
