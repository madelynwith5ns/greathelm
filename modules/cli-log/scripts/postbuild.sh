#!/usr/bin/bash

./build/ghl-info We will now test the various cli-log programs.

./build/ghl-info This is a test of info!
./build/ghl-warn This is a test of warnings!
./build/ghl-error This is a test of errors!
./build/ghl-ok This is a test of when things are ok!

./build/ghl-info We will now explicitly set embedding layers to 0.
GREATHELM_EMBEDDED_LAYERS=0 ./build/ghl-info This is a test of info without embedding!
GREATHELM_EMBEDDED_LAYERS=0 ./build/ghl-warn This is a test of warnings without embedding!
GREATHELM_EMBEDDED_LAYERS=0 ./build/ghl-error This is a test of errors without embedding!
GREATHELM_EMBEDDED_LAYERS=0 ./build/ghl-ok This is a test of when things are ok without embedding!

GREATHELM_EMBEDDED_LAYERS=5 ./build/ghl-info A test of many embedding layers for good measure!

./build/ghl-info This concludes the cli-log test.
