use std::path::PathBuf;

use greathelm::{plugin::GreathelmPlugin, identify::NamespacedIdentifier, version::Version, warning, term::*};

mod builder;
mod generator;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe fn GHPI_PluginInit() -> GreathelmPlugin {
    return GreathelmPlugin { 
        // plugin metadata
        name: "Greathelm Rust Support".into(),
        vendor: "Greathelm".into(),
        description: "Adds experimental Rust support to Greathelm.".into(),
        identifier: NamespacedIdentifier {
            namespace: "io.github.greathelm.ghp".into(),
            identifier: "GHP-Rust".into()
        },
        version: Version::parse("0.1.0-alpha".into()),

        // all of the actual content
        builders: vec![ Box::new(builder::RustBuilder::create()) ],
        generators: vec![ Box::new(generator::RustGenerator::create()) ],
        actions: vec![],
        provides_templates: vec![],

        // ghpi function pointers
        ghpi_create_template: &create_template,
        ghpi_first_time_setup: &first_time_setup,
        ghpi_uninstall: &uninstall,
        ghpi_pluginmessage: &pluginmessage,
        ghpi_plugintextmessage: &plugintextmessage
    }
}

fn create_template(_identifier: NamespacedIdentifier, _path: PathBuf) -> bool {
    // we don't define any templates so we can just return out and not do anything
    true
}

fn first_time_setup() {
    // alpha warning
    warning!("The Greathelm Rust Support (GHP-Rust) plugin is currently in an alpha state.");
    warning!("It is very unstable and not suited to compiling most Rust programs.");
    warning!("Here be dragons!");
}

fn uninstall() {
    // we dont need to do anything on uninstall
}

// we literally do not care about these
fn pluginmessage(_data: &[u8]) {
    warning!("GHP-Rust received a plugin message.");
    warning!("This is likely in error as GHP-Rust does not provide any functionality via plugin messages.");
}
fn plugintextmessage(_text: &str) {
    warning!("GHP-Rust received a plugin text message.");
    warning!("This is likely in error as GHP-Rust does not provide any functionality via plugin messages.");
}
