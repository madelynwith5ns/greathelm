use std::path::Path;

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
            } else if d.starts_with("crates.io/") {
                // crates.io dependencies
                let mut d = d.split_once("crates.io/").unwrap().1;
                let mut is_cargo: bool = false;
                if d.starts_with("cargo/") {
                    d = d.split_once("cargo/").unwrap().1;
                    is_cargo = true;
                }

                if !d.contains("@") {
                    error!("You must specify a version for \x1bccrates.io\x1br dependencies.");
                    error!("Offending dependency: {d}");
                    std::process::exit(1);
                }
                let (d, ver) = d.split_once("@").unwrap();

                let path = format!("lib/crates/lib{d}.rlib");
                let path = Path::new(&path);
                if !path.exists() {
                    info!("Downloading {d}.crate");
                    let curl_incantation = match duct::cmd!(
                        "curl",
                        "-L",
                        format!("https://crates.io/api/v1/crates/{d}/{ver}/download"),
                        "-o",
                        format!("lib/crates/{d}.crate")
                    )
                    .stderr_to_stdout()
                    .run()
                    {
                        Ok(v) => v,
                        Err(_) => {
                            error!("Failed to download crate \x1bc{d}\x1br");
                            error!("Make sure you have \x1bccurl\x1br installed and available in your path");
                            error!("and that you provided a valid crate name and version.");
                            std::process::exit(1);
                        }
                    };
                    if !curl_incantation.status.success() {
                        error!("Failed to download crate \x1bc{d}\x1br");
                        error!("Make sure you have \x1bccurl\x1br installed and available in your path");
                        error!("and that you provided a valid crate name and version.");
                        std::process::exit(1);
                    }

                    info!("Extracting {d}.crate...");

                    let success = match duct::cmd!("tar", "-xf", format!("{d}.crate"))
                        .dir(format!("lib/crates"))
                        .stderr_to_stdout()
                        .run()
                    {
                        Ok(v) => v.status.success(),
                        Err(_) => false,
                    };
                    if !success {
                        error!("Failed to extract {d}.crate");
                        std::process::exit(1);
                    }

                    if is_cargo {
                        info!("{d} is a Cargo dependency. Building with cargo...");
                        let cargo = match duct::cmd!(
                            "cargo",
                            "rustc",
                            "--lib",
                            "--release",
                            "--crate-type",
                            "rlib"
                        )
                        .dir(format!("lib/crates/{d}-{ver}"))
                        .stderr_to_stdout()
                        .run()
                        {
                            Ok(v) => v.status.success(),
                            Err(_) => false,
                        };
                        if !cargo {
                            error!("Failed to build Cargo dependency.");
                            std::process::exit(1);
                        }

                        match std::fs::copy(
                            Path::new(&format!("lib/crates/{d}-{ver}/target/release/lib{d}.rlib")),
                            Path::new(&format!("lib/crates/lib{d}.rlib")),
                        ) {
                            Ok(_) => {
                                ok!("Finished building crates.io+Cargo dependency \x1bc{d}\x1br");
                            }
                            Err(_) => {
                                error!("Failed to copy out dependency \x1bc{d}\x1br");
                                std::process::exit(1);
                            }
                        };
                    } else {
                        error!("Greathelm does not currently support building crates.io dependencies without Cargo.");
                        error!("Please convert dependency \x1bc{d}\x1br to a Cargo dependency by appending \x1bccargo/\x1br after \x1bccrates.io/\x1br.");
                        std::process::exit(1);
                    }
                }

                rustc_args.push("--extern".into());
                rustc_args.push(format!("{d}=lib/crates/lib{d}.rlib"));
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
