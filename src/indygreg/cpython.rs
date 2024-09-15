use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Deserialize, Debug)]
struct Release {
    #[allow(dead_code)]
    tag_name: String,
    #[allow(dead_code)]
    name: String,
    assets: Vec<Asset>,
}

pub async fn get_release(target: String) -> eyre::Result<Vec<(String, Asset)>> {
    let owner = "indygreg";
    let repo = "python-build-standalone";

    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "rust-client")
        .send()
        .await?
        .json::<Release>()
        .await?;

    let assets: Vec<Asset> = response
        .assets
        .into_iter()
        .filter(|asset| asset.name.contains(&target) && asset.name.ends_with("install_only.tar.gz"))
        .collect();

    let versions = assets
        .iter()
        .map(|asset| {
            let version = asset.name.split("-").nth(1).unwrap();
            version.split("+").next().unwrap().to_string()
        })
        .collect::<Vec<String>>();

    let versions = versions
        .iter()
        .map(|version| version.split(".").take(2).collect::<Vec<&str>>().join("."))
        .collect::<Vec<String>>();

    Ok(versions.into_iter().zip(assets).collect())
}
