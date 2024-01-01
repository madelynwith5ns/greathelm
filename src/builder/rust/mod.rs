use std::{io::Write, path::Path};

use crate::{builder::dependency, manifest::ProjectManifest, script, subprocess, term::*};

use super::ProjectBuilder;

/**
 * Built-in builder for Rust projects.
 */
pub struct RustBuilder {}
impl RustBuilder {
    pub fn create() -> Self {
        Self {}
    }
}

impl ProjectBuilder for RustBuilder {
    fn get_name(&self) -> String {
        "Rust".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["rust".into(), "rs".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.greathelm.greathelm".into(),
            identifier: "Rust".into(),
        }
    }
    fn validate(&self, _manifest: &ProjectManifest) -> bool {
        return true;
    }
    fn cleanup(&self, _manifest: &ProjectManifest) {
        warning!("Rust builder does not currently have a cleanup step.");
    }
    fn build(&self, manifest: &ProjectManifest) {
        let build_dir = Path::new("build");
        if !build_dir.exists() {
            match std::fs::create_dir(build_dir) {
                Ok(_) => {}
                Err(_) => {
                    error!("Failed to create build directory. Abort.");
                    return;
                }
            }
        }

        // settings
        let crate_type = manifest.get_string_property("Crate-Type", "bin");
        let project_name = manifest.get_string_property("Project-Name", "Unnamed-Project");
        let executable_name = manifest.get_string_property("Executable-Name", &project_name);
        let opt_level = manifest.get_string_property("Compiler-Opt-Level", "2");
        let rust_edition = manifest.get_string_property("Rust-Edition", "2021");

        script::run_script("prebuild", vec![]);

        // main/lib rs file
        let comp_file = if crate_type == "bin" {
            "src/main.rs"
        } else {
            "src/lib.rs"
        };

        info!(
            "RUSTC \x1bc{}\x1br (\x1bc{}\x1br)",
            project_name, crate_type
        );

        // file decorations for libraries
        let prefix = if crate_type.contains("lib") {
            "lib"
        } else {
            ""
        };
        let suffix = match crate_type.as_str() {
            "rlib" | "dylib" | "lib" => ".rlib",
            "cdylib" => ".so",
            "staticlib" => ".a",
            _ => "",
        };

        // rustc default arguments
        let mut rustc_args = vec![
            "--crate-type".into(),
            format!("{crate_type}"),
            "--emit".into(),
            "link".into(),
            "-o".into(),
            format!("build/{prefix}{executable_name}{suffix}"),
            "--edition".into(),
            rust_edition,
            "-C".into(),
            format!("opt-level={opt_level}"),
        ];

        // dependency directives
        let dependencies = match manifest.directives.get("Dependency") {
            Some(v) => v.to_owned(),
            None => Vec::new(),
        };

        // actually handle deps
        for d in dependencies {
            // vendored deps (rlibs in lib/rlib)
            if d.starts_with("vendored/") {
                let libname = d.split_once("vendored/").unwrap().1;
                rustc_args.push("--extern".into());
                rustc_args.push(format!("{libname}=lib/rlib/lib{libname}.rlib"));
            } else {
                // Greathelm deps (dependencies in the local store)
                let (id, version) = dependency::parse_dependency_notation(d.to_owned());
                let path = match dependency::resolve_dependency(id.clone(), version) {
                    Some(p) => p,
                    None => {
                        error!("Could not resolve a dependency. Abort.");
                        std::process::exit(1);
                    }
                };
                // build the thing
                subprocess::build_project(&path);

                // we have to read the manifest because the export could be a bunch of things here
                // in Rust projects
                let mut depmf = ProjectManifest::new();
                depmf.read_and_append(Path::new(&format!("{}/Project.ghm", path.display())));
                let dep_executable_name =
                    depmf.get_string_property("Executable-Name", id.identifier.as_str());
                let dep_crate_type = depmf.get_string_property("Crate-Type", "rlib");
                let dep_prefix = if dep_crate_type.contains("lib") {
                    "lib"
                } else {
                    ""
                };
                let dep_suffix = match dep_crate_type.as_str() {
                    "rlib" | "dylib" | "lib" => ".rlib",
                    "cdylib" => ".so",
                    "staticlib" => ".a",
                    _ => "",
                };

                // add it to the rustc args
                rustc_args.push("--extern".into());
                rustc_args.push(format!(
                    "{}={}/export/{}{}{}",
                    id.identifier.to_lowercase(),
                    path.display(),
                    dep_prefix,
                    dep_executable_name,
                    dep_suffix
                ));
            }
        }

        // base file
        rustc_args.push(comp_file.into());

        // run rustc
        let rustc = duct::cmd("rustc", rustc_args);
        let rustc = match rustc.stderr_to_stdout().run() {
            Ok(v) => v,
            Err(_) => {
                error!("Failed to invoke compiler.");
                std::process::exit(1);
            }
        };
        if rustc.status.success() {
            ok!(
                "RUSTC \x1bc{}\x1br (\x1bc{}\x1br)",
                project_name,
                crate_type
            );
        } else {
            error!(
                "RUSTC \x1bc{}\x1br (\x1bc{}\x1br)",
                project_name, crate_type
            );
        }

        // export it if its a library project
        if crate_type.contains("lib") {
            info!("Project is a library. Exporting artifact...");
            match std::fs::copy(
                Path::new(&format!("build/{prefix}{executable_name}{suffix}")),
                Path::new(&format!("export/{prefix}{executable_name}{suffix}")),
            ) {
                Ok(_) => {
                    ok!("Done!");
                }
                Err(_) => {
                    error!("Failed to export artifact.");
                }
            };
        }

        script::run_script("postbuild", vec![]);
    }
}
