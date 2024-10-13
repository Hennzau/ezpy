use eyre::OptionExt;

/// Represents a python build package that can be downloaded and extracted.
/*
"cpython-3.13.0-darwin-aarch64-none": {
  "name": "cpython",
  "arch": "aarch64",
  "os": "darwin",
  "libc": "none",
  "major": 3,
  "minor": 13,
  "patch": 0,
  "prerelease": "",
  "url": "https://github.com/indygreg/python-build-standalone/releases/download/20241008/cpython-3.13.0%2B20241008-aarch64-apple-darwin-install_only_stripped.tar.gz",
  "sha256": "7bc4b23590a1e4b41b21b6aae6f92046c1d16d09bc0c1ab81272aa81b55221d1"
},
*/
use crate::metadata::{PackageList, VersionString};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Package {
    /// The architecture of the package (e.g. x86_64, aarch64).
    pub arch: String,

    /// The os family of the package (e.g. windows, linux, darwin).
    pub os: String,

    /// The libc family of the package (e.g. glibc, musl, none).
    pub libc: Option<String>,

    pub major: u32,
    pub minor: u32,
    pub patch: u32,

    pub url: String,
    pub sha256: Option<String>,
}

impl Package {
    pub fn from_string(version: VersionString, packages: PackageList) -> eyre::Result<Self> {
        let (os, arch, family) = system_info()?;
        let key = format!("cpython-{}-{}-{}-{}", version, os, arch, family);

        packages
            .get(&key)
            .ok_or_eyre(eyre::eyre!("Package not found"))
            .cloned()
    }

    pub fn from_version(
        version: crate::metadata::Version,
        packages: PackageList,
    ) -> eyre::Result<Self> {
        let (os, arch, family) = system_info()?;
        let key = format!(
            "cpython-{}.{}.{}-{}-{}-{}",
            version.0, version.1, version.2, os, arch, family
        );

        packages
            .get(&key)
            .ok_or_eyre(eyre::eyre!("Package not found"))
            .cloned()
    }
}

fn system_info() -> eyre::Result<(String, String, String)> {
    // All metadata compatible targets: (from uv metadata generation from indygreg repo)
    //     - darwin-aarch64-none
    //     - darwin-x86_64-none
    //     - linux-aarch64-gnu
    //     - linux-i686-gnu
    //     - linux-x86_64-gnu
    //     - linux-x86_64-musl
    //     - windows-i686-none
    //     - windows-x86_64-none
    //     - linux-powerpc64le-gnu
    //     - linux-s390x-gnu
    //     - linux-armv7-gnueabi

    match (
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::consts::FAMILY,
    ) {
        ("macos", "aarch64", "unix") => Ok((
            "darwin".to_string(),
            "aarch64".to_string(),
            "none".to_string(),
        )),
        ("macos", "x86_64", "unix") => Ok((
            "darwin".to_string(),
            "x86_64".to_string(),
            "none".to_string(),
        )),
        ("linux", "aarch64", "unix") => Ok((
            "linux".to_string(),
            "aarch64".to_string(),
            "gnu".to_string(),
        )),
        ("linux", "i686", "unix") => {
            Ok(("linux".to_string(), "i686".to_string(), "gnu".to_string()))
        }
        ("linux", "x86_64", "unix") => {
            Ok(("linux".to_string(), "x86_64".to_string(), "gnu".to_string()))
        }
        ("linux", "x86_64", "musl") => Ok((
            "linux".to_string(),
            "x86_64".to_string(),
            "musl".to_string(),
        )),
        ("windows", "i686", "windows") => Ok((
            "windows".to_string(),
            "i686".to_string(),
            "none".to_string(),
        )),
        ("windows", "x86_64", "windows") => Ok((
            "windows".to_string(),
            "x86_64".to_string(),
            "none".to_string(),
        )),
        ("linux", "powerpc64le", "unix") => Ok((
            "linux".to_string(),
            "powerpc64le".to_string(),
            "gnu".to_string(),
        )),
        ("linux", "s390x", "unix") => {
            Ok(("linux".to_string(), "s390x".to_string(), "gnu".to_string()))
        }
        ("linux", "armv7", "unix") => Ok((
            "linux".to_string(),
            "armv7".to_string(),
            "gnueabi".to_string(),
        )),
        _ => Err(eyre::eyre!("Unsupported platform")),
    }
}
