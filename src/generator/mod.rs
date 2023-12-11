use std::path::PathBuf;

use crate::identify::NamespacedIdentifier;

pub mod c;
pub mod cpp;
pub mod custom;

pub trait ProjectGenerator {
    fn get_name(&self) -> String;
    fn get_aliases(&self) -> Vec<String>;
    /**
     * Namespaced identifiers are used for builders and generators
     * when the name is ambiguous. For example, if you have two builders
     * installed with the name "C" you will need to specify which one you
     * mean using the identifier. Such as: "greathelm:c" or "example:c"
     */
    fn get_identifier(&self) -> NamespacedIdentifier;
    /**
     * Tells Greathelm whether or not it should create a stub
     * IBHT. This exists incase the IBHT format changes.
     */
    fn should_make_ibht_stub(&self) -> bool;
    fn generate(&self, cwd: PathBuf);
}
