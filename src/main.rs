mod banco;
mod telegram;
mod shortlink;
mod payout;

use teloxide::prelude::*;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting PlantCoin_bot...");

    if let Err(e) = banco::iniciar_banco() {
        log::error!("Erro ao iniciar o banco de dados: {}", e);
        return;
    }
    log::info!("Banco de dados SQLite carregado com sucesso!");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().filter_command::<telegram::Command>().endpoint(telegram::answer))
        .branch(Update::filter_callback_query().endpoint(telegram::cb_handler))
        .branch(Update::filter_message().endpoint(telegram::receber_texto));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
