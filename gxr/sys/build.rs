#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    // build gxr
    use std::path::Path;
    let out_dir = Path::new(&std::env::var("OUT_DIR").unwrap()).to_owned();
    let buildtype = format!("--buildtype={}", std::env::var("PROFILE").unwrap());
    let manifest_dir = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).to_owned();
    std::fs::remove_dir_all(out_dir.join("gxr_build")).unwrap_or(());
    let rc = std::process::Command::new("meson")
        .arg("setup")
        .args([&buildtype, "-Dbackends=openvr,openxr", "--prefix"])
        .arg(out_dir.join("gxr"))
        .arg(out_dir.join("gxr_build"))
        .arg(manifest_dir.join("gxr"))
        .status()
        .unwrap();
    assert!(rc.success());
    std::process::Command::new("meson")
        .args(["compile", "-C"])
        .arg(out_dir.join("gxr_build"))
        .status()
        .unwrap();
    assert!(rc.success());
    std::process::Command::new("meson")
        .args(["install", "-C"])
        .arg(out_dir.join("gxr_build"))
        .status()
        .unwrap();
    assert!(rc.success());
    std::env::set_var(
        "PKG_CONFIG_PATH",
        out_dir.join("gxr").join("lib").join("pkgconfig"),
    );
    println!("cargo:rerun-if-changed=gxr");
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={s}");
        process::exit(1);
    }
}
