use std::collections::{HashMap, HashSet};

use eyre::OptionExt;
use reqwest::Client;
use serde_json::Value;

use crate::package::Package;

pub type VersionString = String;
pub type Version = (u32, u32, u32);
pub type PackageList = HashMap<String, Package>;

async fn download_metadata() -> eyre::Result<Value> {
    let client = Client::new();

    let url = "https://raw.githubusercontent.com/astral-sh/uv/main/crates/uv-python/download-metadata.json";

    let response = client.get(url).send().await?;
    if response.status().is_success() {
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        Ok(json)
    } else {
        Err(eyre::eyre!(
            "Failed to download metadata: {}, check your internet connection",
            response.status()
        ))
    }
}

pub async fn download_versions() -> eyre::Result<HashSet<VersionString>> {
    let json = download_metadata().await?;

    let mut versions = HashSet::new();

    for key in json
        .as_object()
        .ok_or_eyre(eyre::eyre!("Invalid json, the file may be corrupted"))?
        .keys()
    {
        let parts: Vec<&str> = key.split('-').collect();
        let version = parts
            .get(1)
            .cloned()
            .ok_or_eyre(eyre::eyre!("Invalid key, the JSON file may be corrupted"))?;

        versions.insert(version.to_string());
    }

    Ok(versions)
}

pub async fn download_packages() -> eyre::Result<PackageList> {
    let json = download_metadata().await?;

    let mut packages = HashMap::new();

    for (key, value) in json.as_object().ok_or_eyre(eyre::eyre!(
        "Invalid
    json, the file may be corrupted"
    ))? {
        let package = Package {
            arch: value["arch"]
                .as_str()
                .ok_or_eyre(eyre::eyre!("This package doesn't contain an arch key"))?
                .to_string(),
            os: value["os"]
                .as_str()
                .ok_or_eyre(eyre::eyre!("This package doesn't contain an os key"))?
                .to_string(),
            libc: value["libc"].as_str().map(|s| s.to_string()),
            major: value["major"]
                .as_u64()
                .ok_or_eyre(eyre::eyre!("Invalid major"))? as u32,
            minor: value["minor"]
                .as_u64()
                .ok_or_eyre(eyre::eyre!("Invalid minor"))? as u32,
            patch: value["patch"]
                .as_u64()
                .ok_or_eyre(eyre::eyre!("Invalid patch"))? as u32,
            url: value["url"]
                .as_str()
                .ok_or_eyre(eyre::eyre!("Invalid url"))?
                .to_string(),
            sha256: value["sha256"].as_str().map(|s| s.to_string()),
        };

        let key = key.to_string();

        packages.insert(key, package);
    }

    Ok(packages)
}
