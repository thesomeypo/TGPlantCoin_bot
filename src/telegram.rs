use teloxide::{prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup}, utils::command::BotCommands
};

use crate::telegram;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]

pub enum Command {
    Start,
    Username,
    Saldo,
    Colher,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        
        Command::Start => {
            let username = msg.from
                .as_ref()
                .and_then(|user| user.username.clone())
                .unwrap_or_else(|| String::from("membro"));

            let texto = format!("Oi tudo bem @{}", username);

            let keyboard = InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("Ver Username", "btn_username"),
                    InlineKeyboardButton::callback("📊 Saldo", "btn_saldo"),
                    InlineKeyboardButton::callback("💰 Colher", "btn_colher"),
                ]
            ]);

            bot.send_message(msg.chat.id, texto)
                .reply_markup(keyboard)
                .await?;
        }

        Command::Username => {
            let username = msg.from
                .as_ref()
                .and_then(|user| user.username.clone())
                .unwrap_or_else(|| String::from("Desconhecido"));

            let texto = format!("Seu nome de usuario é @{}", username);

            bot.send_message(msg.chat.id, texto).await?;
        }

        Command::Saldo => {
            let saldo = crate::banco::ver_saldo(msg.chat.id.0).unwrap_or(0.0);
            let texto = format!("Seu Saldo atual é de R${}", saldo);

            bot.send_message(msg.chat.id, texto).await?;
        }

        Command::Colher => {
            let lucro_shortlink_1: f64 = 0.005;
            let add_saldo = crate::banco::adicionar_saldo(msg.chat.id.0, lucro_shortlink_1);
            let texto = format!("Parabens voce ganhou R${}", lucro_shortlink_1);

            bot.send_message(msg.chat.id, texto).await?;
            
        }
    }
    Ok(())
}
pub async fn cb_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = q.data.clone() {
        match data.as_str() {
            "btn_username" => {
                let username = q.from.username.clone().unwrap_or_else(|| String::from("Desconhecido"));
                let texto = format!("Seu nome de usuario é @{}", username);

                if let Some(msg) = q.regular_message() {
                    bot.send_message(msg.chat.id, texto).await?;
                }
            }

            "btn_saldo" => {
                if let Some(msg) = q.regular_message() {
                    let saldo = crate::banco::ver_saldo(msg.chat.id.0).unwrap_or(0.0);
                    let texto = format!("Seu Saldo atual é de R${}", saldo);
                    bot.send_message(msg.chat.id, texto).await?;
                }
            }

            "btn_colher" => {
                let lucro_shortlink_1: f64 = 0.005;
                let texto = format!("Parabens voce ganhou R${}!", lucro_shortlink_1);

                if let Some(msg) = q.regular_message() {
                    let add_saldo = crate::banco::adicionar_saldo(msg.chat.id.0, lucro_shortlink_1);
                    bot.send_message(msg.chat.id, texto).await?;
                }
            }
            
            _ => {}
        }
    }
    bot.answer_callback_query(q.id).await?;

    Ok(())
}