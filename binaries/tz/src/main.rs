use indygreg::package::Package;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let packages = indygreg::metadata::download_packages().await?;

    let package = Package::from_string("3.12.6".to_string(), packages)?;

    println!("{:?}", package);

    indygreg::install::download_install(package).await?;

    Ok(())
}
