use crate::{identify::NamespacedIdentifier, state::GreathelmState};

pub mod build;
pub mod checkout;
pub mod import;
pub mod init;
pub mod pkgscript;
pub mod pkgshell;
pub mod remove;
pub mod script;

/**
 * Trait defining actions. Impl this to create command-line actions.
 */
pub trait Action {
    /**
     * Action name. Currently unused. Will be used in a future plugin-tree.
     */
    fn get_name(&self) -> String;
    /**
     * Action aliases. These are the short-names of the action that can be called on the command
     * line.
     */
    fn get_aliases(&self) -> Vec<String>;
    /**
     * The full NamespacedIdentifier of this action. The namespace should be the same as your
     * plugin namespace.
     */
    fn get_identifier(&self) -> NamespacedIdentifier;
    /**
     * Actual action code.
     */
    fn execute(&self, state: &GreathelmState);
}
