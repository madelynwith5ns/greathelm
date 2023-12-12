use crate::{identify::NamespacedIdentifier, state::GreathelmState};

pub mod build;
pub mod init;

pub trait Action {
    fn get_name(&self) -> String;
    fn get_aliases(&self) -> Vec<String>;
    fn get_identifier(&self) -> NamespacedIdentifier;
    fn execute(&self, state: &GreathelmState);
}
