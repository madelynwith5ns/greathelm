# Scripts
Greathelm's functionality can be extended using scripts. All project types can have scripts. Scripts are stored in the `scripts/` directory of a project.

### Available Scripts
**All Project Types**

All projects recognize the following scripts.

- prebuild - Invoked as `prebuild (no arguments)`
- postbuild - Invoked as `postbuild (no arguments)`

**C Projects**

C projects additionally recognize these two additional scripts.

- compiler - Invoked as `compiler $INPUT $OUTPUT` where `$INPUT` is the relative path to the file being compiled and `$OUTPUT` is the relative path to where the compiled object should be placed.
- linker - Invoked as `linker $OUTPUT $INPUTS` where `$OUTPUT` is the relative path to where the linked executable should be placed and where `$INPUTS` is each individual object to be linked as its own argument.

**Custom Projects**

Custom Projects additionally recognize this additional script:

- build - Invoked as `build $OUTPUT` where `$OUTPUT` is the relative path to where the built object should be placed.