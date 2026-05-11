use std::env;

pub async fn encurtador_link(link_limpo: &str) -> Option<String> {
    let api_key = env::var("ENCURTADOR_API")
        .expect("ERRO: Faltou API_KEY");

    let url_api = format!("https://exe.io/api?api={}&url={}", api_key, link_limpo);

    let resposta = reqwest::get(&url_api).await.ok()?;

    let json: serde_json::Value = resposta.json().await.ok()?;

    let link_encurtado = json["shortenedUrl"].as_str()?.to_string();

    Some(link_encurtado)
}