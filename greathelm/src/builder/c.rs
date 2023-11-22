use std::{collections::HashMap, io::Write, path::PathBuf, process::Command, str::FromStr};

use crate::{
    ibht,
    manifest::ProjectManifest,
    term::{error, info, ok, warn}, builder::parallel::ParallelBuild,
};

pub fn build(manifest: ProjectManifest) {
    let cc = match manifest.properties.get("Override-C-Compiler".into()) {
        Some(cc) => cc.to_owned(),
        None => "cc".into(),
    };
    let ld = match manifest.properties.get("Override-C-Linker".into()) {
        Some(ld) => ld.to_owned(),
        None => "cc".into(),
    };
    let opt = match manifest.properties.get("Compiler-Opt-Level".into()) {
        Some(opt) => opt.to_owned(),
        None => "2".into(),
    };
    let artifact = match manifest.properties.get("Executable-Name".into()) {
        Some(artifact) => artifact.to_owned(),
        None => "executable.elf".into(),
    };
    let cflags = match manifest.properties.get("Additional-CC-Flags".into()) {
        Some(cf) => cf.split(",").map(|f| f.to_string()).collect(),
        None => {
            vec![]
        }
    };
    let ldflags = match manifest.properties.get("Additional-LD-Flags".into()) {
        Some(ldf) => ldf.split(",").collect(),
        None => {
            vec![]
        }
    };
    let mut emit = match manifest.properties.get("Emit".into()) {
        Some(emit) => emit.to_owned(),
        None => "binary".into(),
    };
    if emit == "binary" || emit == "executable" {
        info(format!("Emitting an Executable Binary"));
    } else if emit == "shared" || emit == "dylib" {
        info(format!("Emitting a Shared Object"));
    } else {
        warn(format!("Unrecognized EMIT. Defaulting to binary."));
        emit = "binary".into();
    }

    info(format!("Using CC \"{cc}\""));
    info(format!("Using LD \"{ld}\""));

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

    let cpus: usize = std::thread::available_parallelism().unwrap().into();
    info(format!("Building in parallel with {cpus} CPUs..."));
    let mut build = ParallelBuild::new(cpus, rebuild.len());

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
        build.submit(move||{
            let cc_incantation = Command::new(cc.clone())
                .arg("-c")
                .arg("-o")
                .arg(format!(
                        "build/{}-{}.o",
                        str::replace(f.as_path().display().to_string().as_str(), "/", "_"),
                        file
                        ))
                .arg(format!("-O{opt}"))
                .arg("-Wall")
                .arg("-Werror")
                .arg("-Wpedantic")
                .args(cflags.clone())
                .arg(format!("{}", f.display()))
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .output()
                .unwrap();
            print!("{}", String::from_utf8(cc_incantation.stdout).unwrap());
            eprint!("{}", String::from_utf8(cc_incantation.stderr).unwrap());
            std::io::stdout().flush().ok();
            std::io::stderr().flush().ok();
            if cc_incantation.status.success() {
                ok(format!("CC {}", f.display()));
            } else {
                error(format!("CC {}", f.display()));
                std::process::exit(1);
            }
        });

    }

    build.wait();

    info(format!("LD {artifact}"));
    let mut ld_incantation = Command::new(ld.clone());
    let mut prefix = "";
    let mut suffix = "";

    if emit == "shared" || emit == "dylib" {
        prefix = "lib";
        suffix = ".so";
    }

    for dep in &manifest.dependencies {
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
        .arg("-I./lib/include")
        .arg("-L./lib/shared")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    for dep in manifest.dependencies {
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

    if emit == "shared" || emit == "dylib" {
        ld_incantation.arg("-shared");
    }

    if manifest.directives.contains(&"no-link-libc".into()) {
        ld_incantation.arg("-nostdlib");
    }
    if manifest.directives.contains(&"freestanding".into()) {
        ld_incantation.arg("-ffreestanding");
    }

    match manifest.properties.get("C-Linker-Script") {
        Some(script) => {
            ld_incantation.arg("-T");
            ld_incantation.arg(script);
        }
        None => {}
    }

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

    info(format!("Regenerating IBHT for future runs..."));
    ibht::write_ibht();
}
