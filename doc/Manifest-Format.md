# Project Manifest (Project.ghm) Documentation

A Greathelm Manifest (Project.ghm) contains two main types of data:

- Properties:
```ghm
Key=Value
```
- Directives
```ghm
@Dependency libexample
@Directive no-link-libc
```

### Properties
Properties are in the form of Key=Value pairs.

#### The following properties exist on any Greathelm project builder:

- **Project-Name** This specifies the project name.
- **Project-Author** This sets the author. Currently unused.
- **Project-Version** This sets the version. Currently unused.
- **Project-Type** This denotes the type of project. It is very important as without it your project cannot be built. Currently the only valid project type is `C`.

#### C Properties: These properties exist on C projects:
- **Compiler-Opt-Level** Sets the compiler optimization level. Translates to the -O argument.
- **Executable-Name** Sets the name of the compiled executable. If the Emit flag is set to `dylib` or `shared` the builder will add `lib-` and `-.so` to the executable name.
- **Emit** Decides what type of compiled binary should be produced. Valid options are `executable`, `binary` (both mean normal executable binaries), `dylib`, and `shared` (both mean .so shared objects).
- **Override-C-Compiler** Specifies the compiler binary to be used. Defaults to `cc` if unset.
- **Override-C-Linker** Specifies the linker binary to be used. Defaults to `cc` if unset.
- **Additional-CC-Flags** Specifies additional C compiler flags to be used. Separated by a comma (,). Defaults to none.
- **Additional-LD-Flags** Specifies additional C linker flags to be used. Separated by a comma (,). Defaults to none.
- **C-Linker-Script** Specifies a custom linker script to be used.

### @Dependency Directives

#### @Dependency Directives in C Projects
Dependency directives come in three forms:

- `@Dependency !<dependency>` The exclamation mark denotes this as a raw object dependency. These are located in the `lib/obj/` directory. Greathelm automatically appends `.o`. For example if you use `@Dependency !test` it will link `lib/obj/test.o`.

- `@Dependency sys:<dependency>` The `sys:` prefix denotes that the dependency should come from your system instead of the `lib/` directory. This uses `pkg-config`.

- `@Dependency <dependency>` A normal dependency with headers in `lib/include/` and binaries in `lib/shared/`.

### @Directive Directives

@Directives are special instructions given to the builder.

#### @Directives in C Projects
The C builder currently recognizes two @Directives:

- **@Directive no-link-libc** This directive specifies to not link libc into the project. Translates to `-nostdlib`.

- **@Directive freestanding** This directive translates to the `-ffreestanding` cc flag.
