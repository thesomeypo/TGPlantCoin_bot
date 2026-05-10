use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

use chrono::Utc;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]

pub enum Command {
    Start(String),
    Username,
    Saldo,
    Colher,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let user_id = msg.from.as_ref().unwrap().id.0;
    let chat_id = msg.chat.id;

    match cmd {
        Command::Start(codigo) => {
            let username = msg
                .from
                .as_ref()
                .and_then(|user| user.username.clone())
                .unwrap_or_else(|| String::from("membro"));

            let keyboard = InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback("Ver Username", "btn_username"),
                InlineKeyboardButton::callback("📊 Saldo", "btn_saldo"),
                InlineKeyboardButton::callback("💰 Colher", "btn_colher"),
            ]]);

            let codigo_premio = "dinheiro";

            if codigo == codigo_premio {
                let texto = format!(
                    "Voce usou o codigo {} acabou de desbloquear a recompensa",
                    codigo_premio
                );
                bot.send_message(chat_id, texto).await?;
            } else {
                let texto = format!("Oi tudo bem @{}", username);
                bot.send_message(chat_id, texto)
                    .reply_markup(keyboard)
                    .await?;
            }
        }

        Command::Username => {
            let username = msg
                .from
                .as_ref()
                .and_then(|user| user.username.clone())
                .unwrap_or_else(|| String::from("Desconhecido"));

            let texto = format!("Seu nome de usuario é @{}", username);

            bot.send_message(chat_id, texto).await?;
        }

        Command::Saldo => {
            let saldo = crate::banco::ver_saldo(user_id as i64).unwrap_or(0.0);
            let texto = format!("Seu Saldo atual é de R${}", saldo);

            bot.send_message(chat_id, texto).await?;
        }

        Command::Colher => {
            let lucro_shortlink_1: f64 = 0.005;
            let hora_em_segundos: i64 = 3600;
            let agora = Utc::now().timestamp();
            let ultima_coleta: i64 = crate::banco::cooldown(user_id as i64).unwrap_or(0);
            let calculo_se_pode = agora - ultima_coleta;

            if calculo_se_pode >= hora_em_segundos {
                let _ = crate::banco::adicionar_saldo(user_id as i64, lucro_shortlink_1);
                let texto = format!("Parabens voce ganhou R${}", lucro_shortlink_1);
                let _ = crate::banco::atualizar_cooldown(user_id as i64, agora);

                bot.send_message(chat_id, texto).await?;
            } else {
                let faltam_tempo = (hora_em_segundos - calculo_se_pode) / 60;
                let texto = format!(
                    "Não é possivel coletar ainda faltam {} Minutos",
                    faltam_tempo
                );

                bot.send_message(chat_id, texto).await?;
            }
        }
    }
    Ok(())
}

pub async fn cb_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = q.data.clone() {
        let user_id = q.from.id.0;
        let chat_id = q.message.as_ref().map(|m| m.chat().id);

        match data.as_str() {
            "btn_username" => {
                let username = q
                    .from
                    .username
                    .clone()
                    .unwrap_or_else(|| String::from("Desconhecido"));
                let texto = format!("Seu nome de usuario é @{}", username);

                if let Some(c_id) = chat_id {
                    bot.send_message(c_id, texto).await?;
                }
            }

            "btn_saldo" => {
                let saldo = crate::banco::ver_saldo(user_id as i64).unwrap_or(0.0);
                let texto = format!("Seu Saldo atual é de R${}", saldo);

                if let Some(c_id) = chat_id {
                    bot.send_message(c_id, texto).await?;
                }
            }

            "btn_colher" => {
                let lucro_shortlink_1: f64 = 0.005;
                let agora = Utc::now().timestamp();
                let hora_em_segundo: i64 = 3600;
                let ultima_coleta: i64 = crate::banco::cooldown(user_id as i64).unwrap_or(0);
                let calculo_se_pode = agora - ultima_coleta;

                if let Some(c_id) = chat_id {
                    if calculo_se_pode >= hora_em_segundo {
                        let _ = crate::banco::adicionar_saldo(user_id as i64, lucro_shortlink_1);
                        let texto = format!("Parabens voce ganhou R${}!", lucro_shortlink_1);
                        let _ = crate::banco::atualizar_cooldown(user_id as i64, agora);

                        bot.send_message(c_id, texto).await?;
                    } else {
                        let faltam_tempo = (hora_em_segundo - calculo_se_pode) / 60;
                        let texto = format!(
                            "Não é possivel coletar ainda faltam {} Minutos",
                            faltam_tempo
                        );

                        bot.send_message(c_id, texto).await?;
                    }
                }
            }

            _ => {}
        }
    }
    bot.answer_callback_query(q.id).await?;

    Ok(())
}
