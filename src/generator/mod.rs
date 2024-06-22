use std::path::PathBuf;

use crate::identify::NamespacedIdentifier;

pub mod c;
pub mod cpp;
pub mod custom;
pub mod helper;

/**
 * Trait for all project generators.
 *
 */
pub trait ProjectGenerator {
    /**
     * Generator name. This is not necessarily used for calling the generator. It is currently
     * unused but will be used in a future plugin-tree display.
     */
    fn get_name(&self) -> String;
    /**
     * Short names actually used to call this generator.
     */
    fn get_aliases(&self) -> Vec<String>;
    /**
     * Namespaced identifiers are used for builders and generators
     * when the name is ambiguous. For example, if you have two builders
     * installed with the name "C" you will need to specify which one you
     * mean using the identifier. Such as: "greathelm:c" or "example:c"
     */
    fn get_identifier(&self) -> NamespacedIdentifier;
    /**
     * Actually generates the project. This is given the path to create the project in.
     */
    fn generate(&self, cwd: PathBuf);
}
