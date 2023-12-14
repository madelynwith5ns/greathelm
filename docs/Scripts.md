# Scripts
Greathelm's functionality can be extended using scripts. All project types can have scripts. Scripts are stored in the `scripts/` directory of a project.

### Available Scripts
**All Project Types**

All projects recognize the following scripts.

- prebuild - Invoked as `prebuild (no arguments)`. Invoked before any build takes place but after modules are built and fetched.
- postbuild - Invoked as `postbuild (no arguments)`. Invoked after all build steps are complete.
- pre-modules - Invoked as `pre-modules (no arguments)`. Invoked before building and fetching any modules.
- post-modules - Invoked as `post-modules (no arguments)`. Invoked after all modules are built and fetched. Effectively identical to `prebuild`. This script exists for organization purposes.
- module-prebuild - Invoked as `module-prebuild $NAME` where `$NAME` is the module name. Invoked before a module is built.
- module-postbuild - Invoked as `module-postbuild $NAME` where `$NAME` is the module name. Invoked after a module is built.
- module-postfetch - Invoked as `module-postfetch $NAME` where `$NAME` is the module name. Invoked after all files from a module have been copied to their destinations.

**C Projects**

C projects additionally recognize these two additional scripts.

- compiler - Invoked as `compiler $INPUT $OUTPUT` where `$INPUT` is the relative path to the file being compiled and `$OUTPUT` is the relative path to where the compiled object should be placed. Invoked in place of the normal compiler if present.
- linker - Invoked as `linker $OUTPUT $INPUTS` where `$OUTPUT` is the relative path to where the linked executable should be placed and where `$INPUTS` is each individual object to be linked as its own argument. Invoked in place of the normal linker if present.

**Custom Projects**

Custom Projects additionally recognize this additional script:

- build - Invoked as `build $OUTPUT` where `$OUTPUT` is the relative path to where the built object should be placed.
