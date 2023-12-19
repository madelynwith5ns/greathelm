# Greathelm
Greathelm is an extensible, generic software build system.

### Obligatory Alpha Software Warning
Greathelm is alpha software and is still in very early development. Bugs WILL occur. However, when you do find some, please report them! :)

### Generators
Projects are created using Generators. There are currently 3 generators included in Greathelm:
- **C**: `greathelm init --project-type=C`
- **C++**: `greathelm init --project-type=C++`
- **Custom**: `greathelm init --project-type=Custom`

New generators can be added using plugins.

### Builders
Projects are built using builders (crazy). There is a builder for every included generator. 

### Modules
Projects can have `modules` attached to them. Modules are smaller Greathelm projects referenced by a larger one using the `@Module` directive. These projects are built first and the specified files are copied to their location in the larger project's source tree.
Here's an example of using a `libFoo` module in a C project:

```ghm
@Module libFoo lib/shared/libFoo.so:build/libFoo.so lib/include/foo.h:src/foo.h
```

This directive tells Greathelm to first build the `libFoo` project resident within `modules/` and then copy `build/libFoo.so` from its source tree into `lib/shared/libFoo.so` in the parent project, and then do the same with `src/foo.h` to `lib/include/foo.h`.

### Scripts
Scripts can reside within either the `scripts/` folder in a project or globally in the configuration (on Linux, these are located at `$XDG_CONFIG_HOME/greathelm/scripts`).

Some scripts are special and run at certain points in a project's build cycle (like `pre-modules`, `prebuild`, etc.) and any script can be called at any time using `greathelm script <name> <args>`.

### Platform Support

Greathelm only ***targets*** support for Linux platforms however it will *probably* atleast compile and run on any UNIX-like platform (OSX, BSDs, etc.) as long as plugins used are specifically compiled for that platform. Greathelm likely does not work on Windows at all outside of WSL.

### Quickstart
To get started using Greathelm, you first need to install it.
You will need to have [Rust](https://rust-lang.org) installed (specifically, you will need Cargo).

```sh
git clone https://github.com/MadelynWith5Ns/Greathelm && cd Greathelm
cargo build --release
cp target/release/greathelm ~/.local/bin/greathelm
```

**Simple C Hello World**

Create a new folder where you would like this project to reside. In a terminal run:
```sh
greathelm init --project-type=C
```

This will create a bunch of files.

```sh
.
├── IBHT.ghd
├── lib
│   ├── include
│   ├── obj
│   └── shared
├── Project.ghm
└── src
    └── main.c

6 directories, 3 files
```

Let's go over them one by one.

- **IBHT.ghd** The IBHT (Incremental Build Hash Table) is a file that contains a table of relative file paths to their hashes. It is used to determine if a file has changed since a previous build and should be rebuilt. It is currently empty.

- **lib/all the stuffs** This folder contains dependencies for the project with `.so`/`.a` objects going in `lib/shared` and their headers going in `lib/include`. The `lib/obj` directory is for *raw object dependencies* (a.k.a. a random .o you have that you want to link in).

- **Project.ghm** This is the Project manifest. It contains all of the information about your project. For a new C project this will have the following contents:

```ghm
# Greathelm Project Manifest
Project-Name=example
Project-Namespace=com.example
Project-Author=Example Author
Project-Version=0.1.0-alpha
Project-Type=C
Compiler-Opt-Level=2
Executable-Name=example
Emit=binary

Greathelm-Version=0.1.0
```

This sets the name, author, version and type of your project on the first 4 properties, but what are the rest of these for?

`Compiler-Opt-Level` sets the compiler optimization level (wow!). This is the setting passed to your compiler (by default `cc`) as the `-O` flag.

`Executable-Name` is the file within the `build/` directory (that doesn't exist yet!) that will be your final artifact.

`Emit` switches between a normal executable (`binary`, `executable`) and a shared object (`shared`, `dylib`). If a shared object is picked the `Executable-Name` will be prefixed with `lib` and suffixed with `.so`.

`Greathelm-Version` is simply the version of Greathelm used to generate the file. Do not change this.

- **src/main.c** This is just a normal C hello world!

```c
#include <stdio.h>

int main(int argc, char **argv) {
	printf("Hello World!\n");
}
```

**Actually building the project**

Now that we know what all these files do, we can build the project. Building a C project is as simple as running `greathelm build` on the command line. You will now notice two files have appeared in the newly created `build/` directory. `example` (or whatever the `Executable-Name` is) and `src_main.c-2d418e094b2843b55162985beafd1d50.o` (if the C file hasn't been modified). You can run the final project with `./build/example` to see that it does indeed work.

You may also notice that this printed a line similar to the following in your terminal:

```
ⓘ  Building in parallel with 24 CPUs...
```

If you would like to control the number of CPUs (and thus, parallel building jobs) used, either specify `--build-cpus=<num>` on the command line when building OR set `build-cpus` (all lowercase) in one of your manifest files. (Probably set this in `Project.local.ghm` in your project or `$XDG_CONFIG_HOME/greathelm/UserManifest.ghm`).

For a bit more in-depth look at the things you can do, check out `Manifest-Format.md` in the `docs/` directory of this repository. And if you REALLY want to get into the weeds, check out `Plugin-API.md`.

### Libraries Used
Greathelm uses a fairly minimal set of libraries:
- [oconnor663/duct.rs](https://github.com/oconnor663/duct.rs) (MIT licensed)
- [nagisa/rust_libloading](https://github.com/nagisa/rust_libloading) (ISC licensed)
- [stainless-steel/md5](https://github.com/stainless-steel/md5) (MIT/Apache 2.0 licensed, Greathelm uses MIT)
