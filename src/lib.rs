#![deny(clippy::all)]
#![warn(clippy::pedantic)]

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
pub fn get_xplm_flags() -> &'static [&'static str] {
    match get_target_platform() {
        Platform::Windows => &["IBM=1", "LIN=0", "APL=0"],
        Platform::MacOs => &["IBM=0", "LIN=0", "APL=1"],
        Platform::Linux => &["IBM=0", "LIN=1", "APL=0"],
    }
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
    args.push(format!("-I{}/include", acfutils_path.display()));
    args.push(format!(
        "-I{}/{}/include",
        acfutils_path.display(),
        get_short_platform()
    ));
    args.push(format!("-I{}/CHeaders/XPLM", xplane_sdk_path.display()));
    for flag in get_xplm_flags() {
        args.push(format!("-D{flag}"));
    }
    args
}
