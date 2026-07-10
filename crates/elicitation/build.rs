use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=RUSTUP_TOOLCHAIN");
    println!("cargo:rerun-if-env-changed=RUSTC");

    let toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=ELICITATION_RUSTUP_TOOLCHAIN={toolchain}");

    let rustc_version = rustc_version_string().unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=ELICITATION_RUSTC_VERSION={rustc_version}");
}

fn rustc_version_string() -> Option<String> {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let output = Command::new(rustc).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }

    let version = String::from_utf8(output.stdout).ok()?;
    let trimmed = version.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
