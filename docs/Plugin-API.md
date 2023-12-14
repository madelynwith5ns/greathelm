# Plugin API

Greathelm includes a plugin system for implementing new builders, generators, and actions.

Plugins, under the hood, are just shared libraries.

### For Users
If you just want to use a plugin, you can place it in your `(CONFIGROOT)/plugins` folder. This path is usually `~/.config/greathelm/plugins`. The plugin will automatically be loaded and its features made available whenever you run any `greathelm` command.

#### Resolving Name Conflicts
Sometimes two plugins will provide a builder/generator/action with the same name as a builtin feature or another plugin. When this happens, Greathelm will quit with an error message telling you that the name you specified is ambiguous. You can either resolve these per-run by replacing the name of the feature with its full *namespaced identifier*, or you can fix it more permanently with an `@Alias` directive in a manifest file.

**CLI Example** Replacing `greathelm build --Project-Type=C` with `greathelm build --Project-Type=io.github.madelynwith5ns.greathelm:C`

**@Alias Example** Adding `@Alias C=io.github.madelynwith5ns.greathelm:C` to any loaded manifest file.

### For Developers
Creating a plugin aims to be as straightforward as possible. First, you need a Rust project set to produce a `cdylib` (.so/.dll/.dylib). Second, this project needs to depend on `greathelm`. This currently can only be done by setting it as a Git dependency, like so:

```toml
[dependencies]
greathelm = { git = "https://github.com/MadelynWith5Ns/Greathelm", branch = "master" }
```

Once you have this finished, the library needs to export a `#![no_mangle]` `pub unsafe fn GHPI_Init() -> greathelm::plugin::GreathelmPlugin`. This method should at the very least create a plugin struct and return it.
Below is an example of what this might look like:

```rust
#[no_mangle]
pub unsafe fn GHPI_PluginInit() -> GreathelmPlugin {
    let plugin = GreathelmPlugin {
        name: "Example Plugin Name".into(),
        namespace: "com.example.exampleplugin".into(),
        builders: Vec::new(),
        generators: Vec::new(),
        actions: Vec::new()
    };
    return plugin;
}
```

Because Greathelm is still alpha software, this interface is prone to change. Once a stable release rolls around this interface will not change. Any future changes will be added to a new interface such as `GreathelmPluginV2	`.

#### Adding a Generator
Generators add support for creating a project. Creating a generator is not a complicated process. You must create a `struct` that implements `greathelm::generator::ProjectGenerator`. Here is an example of what this might look like.

```rust
pub struct LangGenerator {}
impl LangGenerator {
    pub fn create() -> Self {
        Self {}
    }
}

impl ProjectGenerator for LangGenerator {
    fn get_name(&self) -> String {
        "Language".into()
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "com.example.exampleplugin".into(),
            identifier: "Language".into(),
        }
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["language".into()]
    }
	// the IBHT is a table used to store hashes of files for incremental builds.
	// this specifies whether in running `greathelm init` should Greathelm create
	// a blank IBHT stub.
    fn should_make_ibht_stub(&self) -> bool {
        false
    }
    fn generate(&self, cwd: PathBuf) {
		println!("We are definitely in this moment creating a project file structure...");
    }
}
```

This struct must be added to your `GreathelmPlugin`'s `generators` `Vec` in a `Box`. Like so:
```rust
generators: vec![ Box::new(LangGenerator::create()) ],
```

#### Adding a Builder
The process of adding a builder is largely the same as with generators however the trait to implement is `greathelm::builder::ProjectBuilder` and the `generate` fn is replacd with `build` which takes `&self, manifest: &greathelm::manifest::ProjectManifest`, `validate` which takes the same parameters as build and returns a `bool` stating whether the project is valid to build, and `cleanup` which takes the same parameters as the others.

#### Adding an action
Actions are also largely the same with the trait being `greathelm::action::Action` and the methods being `get_name`, `get_identifier`, `get_aliases`, and finally, `execute` which takes in `&self, state: &greathelm::state::State` and returns nothing.

#### Using ParallelBuild
Greathelm provides the `ParallelBuild` (`greathelm::builder::parallel`) to ease adding multi-threaded compilation. The process of using it is very simple.

- Create a `greathelm::builder::parallel:ParallelBuild` with `ParallelBuild::new(size: usize, total_jobs: usize)` where `size` is the number of CPUs to use and `total_jobs` is the full number of jobs that will be run in this `ParallelBuild` instance.

- Submit all the individual compilation pieces with `ParallelBuild::submit` which takes in a `FnOnce() + Send + 'static` closure.

- Wait for it to complete with `ParallelBuild::wait()`. The build WILL NOT exit unless `wait()` is called. This step is required.

#### Using the Project Manifest
Greathelm Manifests (.ghm) are Greathelm's main configuration format. When Greathelm starts up it loads three manifest files from `(CONFIGROOT)/UserManifest.ghm`, `(PROJECTROOT)/Project.ghm`, and `(PROJECTROOT)/Project.local.ghm`. These manifests are internally all condensed into one `greathelm::manifest::ProjectManifest` instance with each subsequent manifest overriding conflicting keys in previous ones.
Any additional manifests imported with `@Import` are loaded at the location of the `@Import` and override conflicts in previous manifests and are overridden by future ones.

This manifest is simply passed into builders and generators and can be accessed as `state.manifest` in actions. The manifest currently contains 5 properties:

- `manifest.properties` - This is a `HashMap<String,String>` containing all of the properties set in the manifest.
- `manifest.dependencies` - This is a `Vec<String>` of all `@Dependency` dependencies declared.
- `manifest.directives` - This is a `Vec<String>` of all `@Directive` directives declared.
- `manifest.modules` - This is a `Vec<String>` of all `@Module` directives declared. Greathelm implements modules by itself for all project types. You usually do not need to touch these as a plugin author.
- `manifest.aliases` - This is a `Vec<String>` of all `@Alias` directives declared.

#### NamespacedIdentifiers
`greathelm::identify::NamespacedIdentifier`s are how Greathelm deals with two features of the same name (i.e two builders called `C`). They are a `struct` containing two elements:
```rust
pub struct NamespacedIdentifier {
	pub namespace: String,
	pub identifier: String
}
```
Namespaces should be in the format of a reverse domain name (i.e. `io.github.madelynwith5ns.greathelm`, `com.example.exampleplugin`, etc.). `NamespacedIdentifier`s can be converted into a text representation formatted as `namespace:identifier` with `.as_text()` and can be parsed from said format with `::parse_text(text: &String) -> NamespacedIdentifier`.