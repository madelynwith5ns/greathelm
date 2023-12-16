use std::{collections::HashMap, io::Write, path::PathBuf, process::Command, str::FromStr};

use crate::{
    builder::parallel::ParallelBuild,
    ibht,
    manifest::ProjectManifest,
    script,
    term::{error, info, ok, warn},
};

use super::ProjectBuilder;

pub struct CBuilder {}

impl CBuilder {
    pub fn create() -> Self {
        Self {}
    }
}

impl ProjectBuilder for CBuilder {
    fn get_name(&self) -> String {
        "C".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["c".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        return crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "C".into(),
        };
    }
    fn build(&self, manifest: &ProjectManifest) {
        script::run_script("prebuild", vec![]);

        // Settings
        let cc = manifest.get_string_property("Override-C-Compiler", "cc");
        let ld = manifest.get_string_property("Override-C-Linker", "cc");
        let opt = manifest.get_string_property("Compiler-Opt-Level", "2");
        let artifact = manifest.get_string_property("Executable-Name", "binary");
        let cflags = match manifest.properties.get("Additional-CC-Flags".into()) {
            Some(cf) => cf.split(",").map(|f| f.to_string()).collect(),
            None => {
                vec![]
            }
        }; // CFLAGS comma separated
        let ldflags = match manifest.properties.get("Additional-LD-Flags".into()) {
            Some(ldf) => ldf.split(",").collect(),
            None => {
                vec![]
            }
        }; // LDFLAGS comma separated
        let mut emit = manifest.get_string_property("Emit", "binary");
        if emit == "binary" || emit == "executable" {
            info(format!("Emitting an Executable Binary"));
        } else if emit == "shared" || emit == "dylib" {
            info(format!("Emitting a Shared Object"));
        } else {
            warn(format!("Unrecognized EMIT. Defaulting to binary."));
            emit = "binary".into();
        } // output type. binary/executable = normal executable, shared/dylib = .so shared object
        let debug_info = manifest.get_bool_property("debug-info", false);
        let force_full_rebuild = manifest.get_bool_property("force-full-rebuild", true);

        info(format!("Using CC \"{cc}\""));
        info(format!("Using LD \"{ld}\""));

        // find things that changed and should be rebuilt
        info(format!("Hashing project files..."));
        let hashes = ibht::gen_hashtable();
        let ibht = ibht::read_ibht();

        let mut rebuild: HashMap<PathBuf, String> = HashMap::new();
        let mut link: Vec<String> = Vec::new();

        for k in hashes.keys() {
            if k.ends_with(".h") {
                continue;
            }

            let path = PathBuf::from_str(k).unwrap();
            link.push(format!(
                "build/{}-{}.o",
                str::replace(path.display().to_string().as_str(), "/", "_"),
                hashes.get(k).unwrap()
            ));

            if force_full_rebuild {
                info(format!("Running a full rebuild. {k} will be rebuilt."));
                rebuild.insert(
                    PathBuf::from_str(k).unwrap(),
                    hashes.get(k).unwrap().to_owned(),
                );
                continue;
            }

            if !ibht.contains_key(k) {
                info(format!("File {k} changed. It will be rebuilt."));
                rebuild.insert(
                    PathBuf::from_str(k).unwrap(),
                    hashes.get(k).unwrap().to_owned(),
                );
                continue;
            }
            if hashes.get(k).unwrap() != ibht.get(k).unwrap() {
                info(format!("File {k} changed. It will be rebuilt."));
                rebuild.insert(
                    PathBuf::from_str(k).unwrap(),
                    hashes.get(k).unwrap().to_owned(),
                );
            }
        }

        let mut outs: Vec<String> = Vec::new();

        // setup parallel build
        let cpus = manifest.get_usize_property(
            "build-cpus",
            match std::thread::available_parallelism() {
                Ok(v) => v.get(),
                Err(_) => 4, // 4 feels like a safe-ish default
            },
        );

        info(format!("Building in parallel with {cpus} CPUs..."));
        let mut build = ParallelBuild::new(cpus, rebuild.len());

        // actually build all the things
        for f in rebuild.keys() {
            let file = rebuild.get(f).unwrap().to_owned();
            let f = f.to_owned();
            let cc = cc.clone();
            let cflags = cflags.clone();
            let opt = opt.clone();
            outs.push(format!(
                "build/{}.o",
                f.clone().file_name().unwrap().to_string_lossy()
            ));
            build.submit(move || {
                if script::has_script("compiler") {
                    script::run_script(
                        "compiler",
                        vec![
                            format!("{}", f.display()),
                            format!(
                                "build/{}-{}.o",
                                str::replace(f.as_path().display().to_string().as_str(), "/", "_"),
                                file
                            ),
                        ],
                    );
                } else {
                    let mut cc_incantation = Command::new(cc.clone());
                    cc_incantation
                        .arg("-c") // dont link
                        .arg("-o") // output
                        .arg(format!(
                            "build/{}-{}.o",
                            str::replace(f.as_path().display().to_string().as_str(), "/", "_"),
                            file
                        ))
                        .arg(format!("-O{opt}")) // -Oopt from earlier
                        .arg("-Wall") // -Wall
                        .args(cflags.clone()) // the funny cflags
                        .arg(format!("{}", f.display())); // actual file

                    // debug information
                    if debug_info {
                        cc_incantation.arg("-g");
                    }

                    let cc_incantation = cc_incantation
                        .stdout(std::process::Stdio::piped())
                        .stderr(std::process::Stdio::piped())
                        .output()
                        .unwrap();
                    print!("{}", String::from_utf8(cc_incantation.stdout).unwrap());
                    eprint!("{}", String::from_utf8(cc_incantation.stderr).unwrap());
                    std::io::stdout().flush().ok();
                    std::io::stderr().flush().ok();
                    if cc_incantation.status.success() {
                        // result message
                        ok(format!("CC {}", f.display()));
                    } else {
                        error(format!("CC {}", f.display()));
                        std::process::exit(1);
                    }
                }
            });
        }

        // wait for compiling to finish
        build.wait();

        // link
        info(format!("LD {artifact}"));

        let mut prefix = "";
        let mut suffix = "";

        if emit == "shared" || emit == "dylib" {
            prefix = "lib";
            suffix = ".so";
        }

        if script::has_script("linker") {
            let mut args: Vec<String> = vec![format!("build/{prefix}{artifact}{suffix}")];
            args.append(&mut link);
            script::run_script("linker", args);
        } else {
            let dependencies = manifest.directives.get("Dependency").unwrap();
            let mut ld_incantation = Command::new(ld.clone());

            // raw object (.o) dependencies
            for dep in dependencies {
                if !dep.starts_with("!") {
                    continue;
                }
                let dependency = dep.clone().split_off(1);
                link.push(format!("lib/obj/{}.o", dependency));
                info(format!("Linking with raw object {dependency}.o"));
            }

            let ld_incantation = ld_incantation
                .arg("-o")
                .arg(format!("build/{prefix}{artifact}{suffix}"))
                .args(ldflags.clone())
                .args(link)
                .arg("-I./lib/include") // local lib headers
                .arg("-L./lib/shared") // local lib binaries
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped());

            // normal dependencies
            for dep in dependencies {
                if dep.starts_with("!") {
                    continue;
                }
                if dep.starts_with("sys:") {
                    let dep = dep.split_once("sys:").unwrap().1;
                    let pkgconf = Command::new("pkgconf")
                        .arg("--libs")
                        .arg("--cflags")
                        .arg(dep)
                        .output()
                        .unwrap();
                    let dep_ld_flags = String::from_utf8(pkgconf.stdout).unwrap();
                    let dep_ld_flags = dep_ld_flags.split(" ");
                    for flag in dep_ld_flags {
                        if flag == " " {
                            continue;
                        }
                        if flag == "\n" {
                            continue;
                        }
                        ld_incantation.arg(flag);
                    }
                } else {
                    ld_incantation.arg(format!("-l{dep}"));
                }
            }

            // dylibs
            if emit == "shared" || emit == "dylib" {
                ld_incantation.arg("-shared");
            }

            // no standard lib directives
            if manifest
                .directives
                .get("Directive")
                .unwrap()
                .contains(&"no-link-libc".into())
            {
                ld_incantation.arg("-nostdlib");
            }
            if manifest
                .directives
                .get("Directive")
                .unwrap()
                .contains(&"ffreestanding".into())
            {
                ld_incantation.arg("-ffreestanding");
            }

            // custom linker script
            match manifest.properties.get("C-Linker-Script") {
                Some(script) => {
                    ld_incantation.arg("-T");
                    ld_incantation.arg(script);
                }
                None => {}
            }

            // finally, actually link
            let ld_incantation = ld_incantation.output().unwrap();

            print!("{}", String::from_utf8(ld_incantation.stdout).unwrap());
            eprint!("{}", String::from_utf8(ld_incantation.stderr).unwrap());

            std::io::stdout().flush().ok();
            std::io::stderr().flush().ok();

            if ld_incantation.status.success() {
                ok(format!("Project successfully built!"));
            } else {
                error(format!("Project failed to build."));
            }
        }

        info(format!("Regenerating IBHT for future runs..."));
        ibht::write_ibht();
    }

    fn validate(&self, _manifest: &ProjectManifest) -> bool {
        return true;
    }

    fn cleanup(&self, _manifest: &ProjectManifest) {
        error(format!(
            "C builder does not currently include a cleanup step. Aborting."
        ));
    }
}
