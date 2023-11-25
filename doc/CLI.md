# CLI Arguments
All arguments passed to Greathelm that begin with `--` are interpreted as runtime flags. A runtime flag is a property (see `Manifest-Format.md`) that is set at runtime. Properties intended to be set at runtime rather than in the manifest are all lowercase where properties intended to be set in the manifest are in Title-Case.

### Runtime Flags
All runtime flags are below.


#### All project typees
- **project-name***=string* This is used when generating a new project.
- **project-type***=string* This is used when generating a new project.
- **build-cpus***=int* Sets the number of parallel jobs to run when compiling a project.

#### C
- **debug-info***=boolean* This tells the compiler to compile with debug info enabled.
- **force-full-rebuild***=boolean* This rebuilds the project fully. Greathelm will ignore any previously compiled `.o` objects when `--force-full-rebuild=true` is set.
