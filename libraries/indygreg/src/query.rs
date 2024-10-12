use std::collections::HashSet;

use crate::{Package, Version};

pub async fn system_info() -> eyre::Result<(String, String, String)> {
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

pub async fn query_versions() -> eyre::Result<Vec<Version>> {
    let metadata = crate::metadata::download_metadata().await?;

    let mut versions = HashSet::new();
    let mut results = vec![];

    for key in metadata.as_object().unwrap().keys() {
        let parts: Vec<&str> = key.split('-').collect();
        let version = parts
            .get(1)
            .cloned()
            .ok_or_else(|| eyre::eyre!("Invalid key"))?;

        let major_minor_patch: Vec<&str> = version.split('.').collect();
        let major = major_minor_patch.get(0).ok_or_else(|| {
            eyre::eyre!(
                "Invalid
        version"
            )
        })?;
        let minor = major_minor_patch.get(1).ok_or_else(|| {
            eyre::eyre!(
                "Invalid
        version"
            )
        })?;
        let patch = major_minor_patch.get(2).ok_or_else(|| {
            eyre::eyre!(
                "Invalid
        version"
            )
        })?;

        if !major.parse::<u32>().is_ok()
            || !minor.parse::<u32>().is_ok()
            || !patch.parse::<u32>().is_ok()
        {
            continue;
        }

        let result = Version {
            major: major.parse()?,
            minor: minor.parse()?,
            patch: patch.parse()?,
        };

        if !versions.contains(&version) {
            versions.insert(version);
            results.push(result);
        }
    }

    Ok(results)
}

pub async fn query_package(version: Version) -> eyre::Result<Package> {
    let (os, arch, abi) = system_info().await?;
    let metadata = crate::metadata::download_metadata().await?;

    let metadata_key = format!(
        "cpython-{}.{}.{}-{}-{}-{}",
        version.major, version.minor, version.patch, os, arch, abi
    );

    let package = metadata
        .get(&metadata_key)
        .ok_or_else(|| eyre::eyre!("Package not found"))?;

    let prerelease = package["prerelease"].as_str().map(|s| s.to_string());

    let url = package["url"].as_str().map(|s| s.to_string());

    let sha256 = package["sha256"].as_str().map(|s| s.to_string());

    if url.is_none() {
        return Err(eyre::eyre!("URL not found"));
    }

    let package = Package {
        name: "cpython".to_string(),
        arch: arch.clone(),
        os: os.clone(),
        libc: abi.clone(),
        major: version.major,
        minor: version.minor,
        patch: version.patch,
        prerelease,
        url: url.unwrap(),
        sha256,
    };

    Ok(package)
}
