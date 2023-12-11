use crate::{identify::NamespacedIdentifier, manifest::ProjectManifest};

pub mod c;
pub mod cpp;
pub mod custom;
pub mod parallel;

pub trait ProjectBuilder {
    fn get_name(&self) -> String;
    fn get_aliases(&self) -> Vec<String>;
    /**
     * Namespaced identifiers are used for builders and generators
     * when the name is ambiguous. For example, if you have two builders
     * installed with the name "C" you will need to specify which one you
     * mean using the identifier. Such as: "greathelm:c" or "example:c"
     */
    fn get_identifier(&self) -> NamespacedIdentifier;
    fn build(&self, manifest: &ProjectManifest);
    /**
     * Validate is called before build in an effort to ensure the project
     * is in working order to be built. This is where you would put things
     * like a code analyzer. Should return false if the project is invalid.
     */
    fn validate(&self, manifest: &ProjectManifest) -> bool;
    /**
     * This is called by the `greathelm clean` command.
     * Use this space to clean up after your build.
     * In the C builder this would remove all stale objects in the build
     * directory.
     */
    fn cleanup(&self, manifest: &ProjectManifest);
}
