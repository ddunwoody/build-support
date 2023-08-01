#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Windows,
    MacOs,
    Linux,
}

impl From<String> for Platform {
    fn from(value: String) -> Self {
        if value == "macos" {
            Platform::MacOs
        } else if value == "windows" {
            Platform::Windows
        } else if value == "linux" {
            Platform::Linux
        } else {
            panic!("Unsupported target: {value}");
        }
    }
}

#[must_use]
pub fn get_target_platform() -> Platform {
    Platform::from(std::env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set"))
}

#[must_use]
pub fn get_xplm_flags() -> Vec<&'static str> {
    let mut platform_flags = match get_target_platform() {
        Platform::Windows => vec!["IBM=1", "LIN=0", "APL=0"],
        Platform::MacOs => vec!["IBM=0", "LIN=0", "APL=1"],
        Platform::Linux => vec!["IBM=0", "LIN=1", "APL=0"],
    };
    platform_flags.extend([
        "XPLM200=1",
        "XPLM210=1",
        "XPLM300=1",
        "XPLM301=1",
        "XPLM303=1",
        "XPLM400=1",
    ]);
    platform_flags
}

#[must_use]
pub fn get_short_platform() -> &'static str {
    match get_target_platform() {
        Platform::Windows => "mingw64",
        Platform::MacOs => "mac64",
        Platform::Linux => "lin64",
    }
}

#[must_use]
pub fn get_acfutils_bindgen_clang_args(
    acfutils_path: &std::path::Path,
    xplane_sdk_path: &std::path::Path,
) -> Vec<String> {
    let mut args = Vec::new();
    for include in get_acfutils_includes(acfutils_path, xplane_sdk_path) {
        args.push(format!("-I{}", include.display()));
    }
    for flag in get_xplm_flags() {
        args.push(format!("-D{flag}"));
    }
    args
}

#[must_use]
pub fn get_acfutils_includes(
    acfutils_path: &std::path::Path,
    xplane_sdk_path: &std::path::Path,
) -> Vec<PathBuf> {
    vec![
        acfutils_path.join("include"),
        acfutils_path.join(get_short_platform()).join("include"),
        xplane_sdk_path.join("CHeaders/XPLM"),
        xplane_sdk_path.join("CHeaders/Widgets"),
    ]
}
