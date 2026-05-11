use reqwest::Client;
use std::env;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResponseFaucetPay {
    pub status: i32,
    pub message: String,
}

pub async fn pagar(email_destino: &str, valor_dolar: f64) -> Result<String, String> {
    let api_key = env::var("FAUCETPAY_API").expect("API_KEY não encontrada");
    let valor_satoshi = (valor_dolar * 100_000_000.0) as i64;
    let paramenters = [
        ("api_key", api_key),
        ("currency", "USDT".to_string()),
        ("amount", valor_satoshi.to_string()),
        ("to", email_destino.to_string()),
    ];

    let client = Client::new();
    let resposta = client.post("https://faucetpay.io/api/v1/send")
        .form(&paramenters)
        .send()
        .await;

    match resposta {
        Ok(res) => {
            if let Ok(json) = res.json::<ResponseFaucetPay>().await {
                if json.status == 200 {
                    return Ok("Pagamento enviado com sucesso!".to_string());
                } else {
                    return Err(format!("ERRO {}", json.message));
                }
            }
            Err("Erro ao ler a resposta da FaucetPay.".to_string())
        }
        Err(_) => Err("Erro de conexão com a internet.".to_string()),
    }
}