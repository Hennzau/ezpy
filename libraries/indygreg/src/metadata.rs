use eyre::Result;
use reqwest::Client;
use serde_json::Value;

pub async fn download_metadata() -> Result<Value> {
    // Créer un client HTTP
    let client = Client::new();

    // L'URL du fichier JSON
    let url = "https://raw.githubusercontent.com/astral-sh/uv/d12d569f24150d3e78dce87a9abf2313b9edac06/crates/uv-python/download-metadata.json";

    // Envoyer une requête GET
    let response = client.get(url).send().await?;

    // Vérifier que la requête a réussi
    if response.status().is_success() {
        // Lire le corps de la réponse en tant que texte
        let body = response.text().await?;

        // Parser le corps en tant que JSON
        let json: Value = serde_json::from_str(&body)?;

        // Afficher le dictionnaire JSON
        return Ok(json);
    } else {
        // Afficher le code d'erreur
        return Err(eyre::eyre!(
            "Failed to download metadata: {}",
            response.status()
        ));
    }
}
