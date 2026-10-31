use std::env::var;
use std::path::{Path, PathBuf};

fn main() {
    let dir = var("CARGO_MANIFEST_DIR").unwrap();
    let family = var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let env = var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    println!("cargo:rerun-if-changed=code/librecomp.a");
    println!("cargo:rerun-if-changed=assets/icone.ico");
    if env == "msvc" {
        // MSVC's linker looks for recomp.lib, the same archive by its name
        let out = var("OUT_DIR").unwrap();
        std::fs::copy(format!("{dir}/code/librecomp.a"), format!("{out}/recomp.lib")).unwrap();
        println!("cargo:rustc-link-search=native={out}");
    } else {
        println!("cargo:rustc-link-search=native={dir}/code");
    }
    println!("cargo:rustc-link-lib=static=recomp");
    if family == "unix" {
        println!("cargo:rustc-link-lib=m");
    }
    // the code runs on the main thread, whose stack is 8 MiB on Linux and
    // 1 MiB on Windows unless it asks for more
    if family == "windows" {
        let stack = 8 << 20;
        match env.as_str() {
            "msvc" => println!("cargo:rustc-link-arg-bins=/STACK:{stack}"),
            _ => println!("cargo:rustc-link-arg-bins=-Wl,--stack,{stack}"),
        }
        if env == "msvc" {
            embed_windows_icon(&var("CARGO_MANIFEST_DIR").unwrap());
        }
    }
}

/// Compile the supplied icon into the DQ8Recomp executable's resources.
fn embed_windows_icon(manifest_dir: &str) {
    let icon = Path::new(manifest_dir).join("assets").join("icone.ico");
    if !icon.is_file() {
        panic!("the DQ8 launcher icon is missing: {}", icon.display());
    }

    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    let rc_file = out_dir.join("DQ8Recomp.rc");
    let res_file = out_dir.join("DQ8Recomp.res");
    let icon_path = icon.to_string_lossy().replace('\\', "/");
    std::fs::write(
        &rc_file,
        format!("IDI_ICON1 ICON \"{icon_path}\"\r\n"),
    )
    .expect("write the DQ8 icon resource script");

    let rc = find_resource_compiler().unwrap_or_else(|| {
        panic!("Windows resource compiler rc.exe was not found; install the Windows SDK to embed the DQ8 icon")
    });
    let status = std::process::Command::new(&rc)
        .arg("/nologo")
        .arg("/fo")
        .arg(&res_file)
        .arg(&rc_file)
        .status()
        .unwrap_or_else(|error| panic!("could not run {}: {error}", rc.display()));
    if !status.success() {
        panic!("Windows resource compiler failed to embed {}", icon.display());
    }
    println!("cargo:rustc-link-arg-bins={}", res_file.display());
}

fn find_resource_compiler() -> Option<PathBuf> {
    if let Some(sdk_dir) = std::env::var_os("WindowsSdkDir") {
        let version = std::env::var_os("WindowsSDKVersion").unwrap_or_default();
        let version = version.to_string_lossy().trim_matches(['\\', '/']).to_owned();
        let candidate = PathBuf::from(sdk_dir)
            .join("bin")
            .join(version)
            .join("x64")
            .join("rc.exe");
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    let kits = Path::new(r"C:\Program Files (x86)\Windows Kits\10\bin");
    let mut candidates: Vec<_> = std::fs::read_dir(kits)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path().join("x64").join("rc.exe"))
        .filter(|path| path.is_file())
        .collect();
    candidates.sort();
    candidates.pop()
}
