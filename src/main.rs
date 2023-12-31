use std::{path::PathBuf, str::FromStr};

use crate::term::*;
use action::Action;
use builder::ProjectBuilder;
use generator::ProjectGenerator;
use identify::NamespacedIdentifier;
use plugin::PluginInfo;
use state::GreathelmState;

mod action;
mod builder;
mod config;
mod generator;
mod ibht;
mod identify;
mod manifest;
mod module;
mod plugin;
mod script;
mod state;
mod store;
mod subprocess;
mod template;
mod term;
mod util;
mod version;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut action = match args.get(1) {
        Some(arg) => arg.clone(),
        None => {
            error!("Usage: greathelm <action> [args]");
            std::process::exit(0);
        }
    };

    config::ensure_config_dirs();

    let mut builders: Vec<Box<dyn ProjectBuilder>> = Vec::new();
    let mut generators: Vec<Box<dyn ProjectGenerator>> = Vec::new();
    let mut manifest = manifest::ProjectManifest::new();
    let mut actions: Vec<Box<dyn Action>> = Vec::new();

    // user manifest
    let path = PathBuf::from_str(
        format!(
            "{}/UserManifest.ghm",
            config::get_config_base_dir().to_str().unwrap()
        )
        .as_str(),
    )
    .unwrap();
    if path.exists() {
        manifest.read_and_append(&path);
    }
    let path = PathBuf::from_str("Project.ghm").unwrap();
    if path.exists() {
        manifest.read_and_append(&path);
    }
    let path = PathBuf::from_str("Project.local.ghm").unwrap();
    if path.exists() {
        manifest.read_and_append(&path);
    }

    manifest.append_from_cli_args(std::env::args().collect());

    let aliases = manifest.get_aliases_map();

    let mut pt = match manifest.properties.get("Project-Type") {
        Some(t) => t.clone(),
        None => "%".into(),
    };
    if pt != "" {
        for a in aliases.keys() {
            if &pt == a {
                pt = aliases.get(a).unwrap().clone();
            }
        }
    }
    manifest.properties.insert("Project-Type".into(), pt);

    // builtins
    builders.push(Box::new(builder::c::CBuilder::create()));
    builders.push(Box::new(builder::cpp::CPPBuilder::create()));
    builders.push(Box::new(builder::custom::CustomBuilder::create()));

    generators.push(Box::new(generator::c::CGenerator::create()));
    generators.push(Box::new(generator::cpp::CPPGenerator::create()));
    generators.push(Box::new(generator::custom::CustomGenerator::create()));

    actions.push(Box::new(action::init::InitAction::create()));
    actions.push(Box::new(action::build::BuildAction::create()));
    actions.push(Box::new(action::script::ScriptAction::create()));
    actions.push(Box::new(action::import::ImportAction::create()));
    actions.push(Box::new(action::checkout::CheckoutAction::create()));
    actions.push(Box::new(action::remove::RemoveAction::create()));
    actions.push(Box::new(action::pkgshell::PackageShell::create()));
    actions.push(Box::new(action::pkgscript::PackageScript::create()));
    actions.push(Box::new(action::new::NewAction::create()));
    actions.push(Box::new(action::install::InstallAction::create()));
    actions.push(Box::new(action::uninstall::UninstallAction::create()));

    // load plugins here..
    let plugins = plugin::load_plugins();
    let mut infos: Vec<PluginInfo> = Vec::new();
    for plugin in plugins {
        infos.push(plugin.as_info());

        for b in plugin.builders {
            builders.push(b);
        }
        for g in plugin.generators {
            generators.push(g);
        }
        for a in plugin.actions {
            actions.push(a);
        }
    }

    let state = GreathelmState {
        builders: builders,
        generators: generators,
        manifest: manifest,
        actions: actions,
        plugins: infos,
        cli_args: std::env::args().collect(),
    };

    let aliases = state.manifest.get_aliases_map();
    for a in aliases.keys() {
        if &action == a {
            action = aliases.get(a).unwrap().clone();
        }
    }
    let mut use_action: Option<&Box<dyn Action>> = None;
    let namespaced = match NamespacedIdentifier::parse_text(&action) {
        Some(v) => v,
        None => NamespacedIdentifier {
            namespace: "?".into(),
            identifier: action.clone(),
        },
    };
    for a in &state.actions {
        if a.get_aliases().contains(&action.to_lowercase()) {
            if use_action.is_some() {
                error!(
                    "Action name \x1bc{}\x1br is ambiguous in your configuration.",
                    action
                );
                error!("Please specify which one you would like to use.");
                error!("Example: \x1bcgreathelm <full.namespaced:Identifier>\x1br");
                std::process::exit(1);
            } else {
                use_action = Some(a);
            }
        } else if namespaced.namespace != "unnamespaced" && a.get_identifier() == namespaced {
            use_action = Some(a);
        }
    }

    match use_action {
        Some(a) => {
            a.execute(&state);
        }
        None => {
            error!("Action \x1bc{action}\x1br could not be resolved.");
            error!("Are you missing a plugin?");
        }
    }
}
