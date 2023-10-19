/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Windows,
    MacOs,
    Linux,
}

impl Platform {
    #[must_use]
    pub fn short(&self) -> &str {
        match self {
            Platform::Windows => "mingw64",
            Platform::MacOs => "mac64",
            Platform::Linux => "lin64",
        }
    }

    fn xplm_flags(self) -> Vec<String> {
        let mut flags = match self {
            Platform::Windows => vec!["-DIBM=1", "-DLIN=0", "-DAPL=0"],
            Platform::MacOs => vec!["-DIBM=0", "-DLIN=0", "-DAPL=1"],
            Platform::Linux => vec!["-DIBM=0", "-DLIN=1", "-DAPL=0"],
        };
        flags.extend([
            "-DXPLM200=1",
            "-DXPLM210=1",
            "-DXPLM300=1",
            "-DXPLM301=1",
            "-DXPLM303=1",
            "-DXPLM400=1",
        ]);
        flags.iter().map(ToString::to_string).collect()
    }
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
pub fn get_acfutils_cflags(
    platform: Platform,
    acfutils_path: &std::path::Path,
    xplane_sdk_path: &std::path::Path,
) -> Vec<String> {
    let mut args = vec![
        format!("-I{}/include", acfutils_path.display()),
        format!("-I{}/{}/include", acfutils_path.display(), platform.short()),
        format!("-I{}/CHeaders/XPLM", xplane_sdk_path.display()),
        format!("-I{}/CHeaders/Widgets", xplane_sdk_path.display()),
    ];
    args.extend(
        [
            "-std=c99",
            "-DGLEW_MX",
            "-DCURL_STATICLIB",
            "-DPCRE2_STATIC",
            "-DPCRE2_CODE_UNIT_WIDTH=8",
        ]
        .iter()
        .map(ToString::to_string),
    );
    let platform_args = match platform {
        Platform::Windows => vec!["-D_WIN32_WINNT=0x0600", "-DLIBXML_STATIC"],
        Platform::MacOs => vec!["-DLACF_GLEW_USE_NATIVE_TLS=0"],
        Platform::Linux => vec!["-D_GNU_SOURCE"],
    };
    args.extend(platform_args.iter().map(ToString::to_string));
    args.extend(platform.xplm_flags());
    args
}

#[must_use]
pub fn get_acfutils_libs(platform: Platform) -> Vec<String> {
    let mut libs = vec![
        "acfutils", "lzma", "iconv", "cairo", "pixman-1", "freetype", "png16", "shp", "proj",
    ];

    if platform == Platform::Windows {
        libs.push("glew32mx");
    } else {
        libs.push("GLEWmx");
    }

    libs.extend(vec!["curl", "ssl", "crypto"]);

    if platform == Platform::Windows {
        libs.push("gdi32");
    }

    libs.push("z");

    if platform == Platform::Windows {
        libs.extend(vec!["ws2_32", "crypt32"]);
    }

    if platform == Platform::Linux {
        libs.push("pthread");
    }

    libs.extend(vec!["xml2", "pcre2-8"]);

    if platform == Platform::Windows {
        libs.extend(vec!["dbghelp", "psapi", "ssp", "bcrypt", "winmm"]);
    }

    libs.push(match platform {
        Platform::Windows => "opengl32",
        Platform::MacOs => "framework=OpenGL",
        Platform::Linux => "GL",
    });

    libs.iter().map(ToString::to_string).collect()
}
