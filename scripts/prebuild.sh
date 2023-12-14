#!/usr/bin/bash

echo Running rustfmt
find src/ -type f | xargs rustfmt --edition 2021
