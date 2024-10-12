use indygreg::query::{self, query_package, query_versions};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // let metadata = indygreg::metadata::download_metadata().await?;

    // let mut already_visited = std::collections::HashSet::new();

    // for key in metadata.as_object().unwrap().keys() {
    //     // for each key, remove the first two elements between "-" and "-"
    //     let mut parts: Vec<&str> = key.split("-").collect();
    //     parts.remove(0);
    //     parts.remove(0);

    //     // Assemble the parts into a string
    //     let name = parts.join("-");

    //     // If the name is not in the already_visited set, add it
    //     if !already_visited.contains(&name) {
    //         already_visited.insert(name.clone());
    //         println!("{}", name);
    //     }
    // }

    let version = query_versions().await?.first().cloned().unwrap();
    let package = query_package(version).await?;

    println!("{:?}", package);

    Ok(())
}
