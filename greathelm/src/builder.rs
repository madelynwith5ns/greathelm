use std::{path::{Path, PathBuf}, str::FromStr, process::Command, io::Stdout, collections::HashMap};

use crate::{projectmanifest::ProjectManifest, term::{error, info, ok}, ibht};

pub fn build(manifest: ProjectManifest) {
    let project_type = manifest.properties.get("Project-Type").unwrap();

    let build_dir = Path::new("build");
    if !build_dir.exists() {
        match std::fs::create_dir(build_dir) {
            Ok(_) => {},
            Err(e) => {
                error(format!("Failed to create project build directory! Error is below:"));
                eprintln!("{}",e);
                std::process::exit(1);
            }
        }
    }

    match project_type.as_str() {
        "C" => {
            c_builder(manifest);
        }
        _ => {
            error(format!("An invalid project type was passed to the builder."));
            return;
        },
    }
}

pub fn c_builder(manifest: ProjectManifest) {
    let cc = match manifest.properties.get("Override-C-Compiler".into()) {
        Some(cc) => { cc.to_owned() },
        None => { "cc".into() },
    };
    let ld = match manifest.properties.get("Override-C-Linker".into()) {
        Some(ld) => { ld.to_owned() },
        None => { "cc".into() },
    };
    let opt = match manifest.properties.get("C-Opt-Level".into()) {
        Some(opt) => { opt.to_owned() },
        None => { "2".into() }
    };
    let artifact = match manifest.properties.get("Executable-Name".into()) {
        Some(artifact) => { artifact.to_owned() },
        None => { "executable.elf".into() }
    };

    info(format!("Using CC \"{cc}\""));
    info(format!("Using LD \"{ld}\""));

    info(format!("Hashing project files..."));
    let hashes = ibht::gen_hashtable();
    let ibht = ibht::read_ibht();

    let mut rebuild: HashMap<PathBuf,String> = HashMap::new();
    let mut link: Vec<String> = Vec::new();

    for k in hashes.keys() {
        if k.ends_with(".h") { continue; }

        let path = PathBuf::from_str(k).unwrap();
        link.push(format!("build/{}-{}.o", path.file_name().unwrap().to_string_lossy(),hashes.get(k).unwrap()));

        if !ibht.contains_key(k) { 
            info(format!("File {k} changed. It will be rebuilt."));
            rebuild.insert(PathBuf::from_str(k).unwrap(),hashes.get(k).unwrap().to_owned());
            continue;
        } 
        if hashes.get(k).unwrap() != ibht.get(k).unwrap() {
            info(format!("File {k} changed. It will be rebuilt."));
            rebuild.insert(PathBuf::from_str(k).unwrap(),hashes.get(k).unwrap().to_owned());
        }
    }

    let mut outs: Vec<String> = Vec::new();

    for f in rebuild.keys() {
        info(format!("CC {}", f.display()));
        let cc_incantation = Command::new(cc.clone())
            .arg("-c")
            .arg("-o")
            .arg(format!("build/{}-{}.o", f.file_name().unwrap().to_string_lossy(),rebuild.get(f).unwrap()))
            .arg(format!("-O{opt}"))
            .arg("-Wall")
            .arg("-Werror")
            .arg("-Wpedantic")
            .arg(format!("{}", f.display()))
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn().unwrap().wait().unwrap();
        outs.push(format!("build/{}.o",f.file_name().unwrap().to_string_lossy()));
        if cc_incantation.success() {
            ok(format!("CC {}", f.display()));
        } else {
            error(format!("CC {}", f.display()));
            std::process::exit(1);
        }
    }

    info(format!("LD {artifact}"));
    let ld_incantation = Command::new(ld.clone())
        .arg("-o")
        .arg(format!("build/{artifact}"))
        .args(link)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if ld_incantation.success() {
        ok(format!("Project successfully built!"));
    } else {
        error(format!("Project failed to build."));
    }

    info(format!("Regenerating IBHT for future runs..."));
    ibht::write_ibht();
}
