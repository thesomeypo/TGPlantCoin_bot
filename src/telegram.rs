use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

use chrono::Utc;
use tokio::time::{Duration, sleep};
use uuid::*;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]

pub enum Command {
    Start(String),
    Username,
    Saldo,
    Colher,
    Saque,
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
                InlineKeyboardButton::callback("🏧 Sacar", "btn_sacar")
            ]]);

            let codigo_premio = crate::banco::ver_codigo(user_id as i64).unwrap_or(None);

            if Some(codigo.clone()) == codigo_premio {
                let texto = format!("Voce usou o codigo acabou de desbloquear a recompensa");
                bot.send_message(chat_id, texto).await?;

                let lucro_shortlink_1: i64 = 27;
                let texto2 = format!("Parabéns voce colheu {} 🍎 Frutas!", lucro_shortlink_1);
                let _ = crate::banco::adicionar_saldo(user_id as i64, lucro_shortlink_1);
                let _ = crate::banco::limpar_codigo(&codigo);
                bot.send_message(chat_id, texto2).await?;
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
            let saldo = crate::banco::ver_saldo(user_id as i64).unwrap_or(0);
            let texto = format!("Seu Saldo atual é de {} 🍎 Frutas", saldo);

            bot.send_message(chat_id, texto).await?;
        }

        Command::Colher => {
            let codigo_recompensa = uuid::Uuid::new_v4().to_string();
            let agora = Utc::now().timestamp();
            let ultima_coleta = crate::banco::cooldown(user_id as i64).unwrap_or(0);
            let hora_em_segundo: i64 = 3600;
            let calculo_se_pode = agora - ultima_coleta;

            if calculo_se_pode >= hora_em_segundo {
                let _ = crate::banco::salvar_codigo(user_id as i64, &codigo_recompensa, agora);
                let _ = crate::banco::atualizar_cooldown(user_id as i64, agora);
                let link_codigo = format!(
                    "https://t.me/PlantCoincryptobot?start={}",
                    codigo_recompensa
                );
                let link_final = crate::shortlink::encurtador_link(&link_codigo)
                    .await
                    .unwrap_or(link_codigo);
                let texto = format!(
                    "Clique nesse link para ganhar sua recompensa:\n{}",
                    link_final
                );
                bot.send_message(chat_id, texto).await?;

                let codigo_para_deletar = codigo_recompensa.clone();

                tokio::spawn(async move {
                    sleep(Duration::from_secs(3600)).await;
                    let _ = crate::banco::limpar_codigo(&codigo_para_deletar);
                });
            } else {
                let faltam_tempo = (hora_em_segundo - calculo_se_pode) / 60;
                let texto = format!(
                    "Não é possivel coletar ainda. Faltam {} Minutos.",
                    faltam_tempo
                );
                bot.send_message(chat_id, texto).await?;
            }
        }

        Command::Saque => {
            let saque_minimo: i64 = 2000;
            let saldo_atual = crate::banco::ver_saldo(user_id as i64).unwrap_or(0);
            let agora = Utc::now().timestamp();

            if saldo_atual >= saque_minimo {
                let _ = crate::banco::saque(user_id as i64, agora);
                let texto = format!(
                    "🏦 *Saque Iniciado!*\n
                Seu saldo: {} 🍎 Frutas\n\n
                Digite no chat a QUANTIDADE que deseja sacar (Somente números):",
                    saldo_atual
                );

                bot.send_message(chat_id, texto).await?;
            } else {
                let texto = format!(
                    "Saldo insuficiente, o saque minimo é {} 🍎 Frutas",
                    saque_minimo
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
                let saldo = crate::banco::ver_saldo(user_id as i64).unwrap_or(0);
                let texto = format!("Seu Saldo atual é de {} 🍎 Frutas", saldo);

                if let Some(c_id) = chat_id {
                    bot.send_message(c_id, texto).await?;
                }
            }

            "btn_colher" => {
                let codigo_recompensa = Uuid::new_v4().to_string();
                let agora = Utc::now().timestamp();
                let hora_em_segundo: i64 = 3600;
                let ultima_coleta: i64 = crate::banco::cooldown(user_id as i64).unwrap_or(0);
                let calculo_se_pode = agora - ultima_coleta;

                if let Some(c_id) = chat_id {
                    if calculo_se_pode >= hora_em_segundo {
                        let _ =
                            crate::banco::salvar_codigo(user_id as i64, &codigo_recompensa, agora);
                        let _ = crate::banco::atualizar_cooldown(user_id as i64, agora);
                        let link_codigo = format!(
                            "https://t.me/PlantCoincryptobot?start={}",
                            codigo_recompensa
                        );
                        let link_final = crate::shortlink::encurtador_link(&link_codigo)
                            .await
                            .unwrap_or(link_codigo);
                        let texto = format!(
                            "Clique nesse link para ganhar sua recompensa {}",
                            link_final
                        );
                        bot.send_message(c_id, texto).await?;

                        let codigo_para_deletar = codigo_recompensa;

                        tokio::spawn(async move {
                            sleep(Duration::from_secs(3600)).await;
                            let _ = crate::banco::limpar_codigo(&codigo_para_deletar);
                        });
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

            "btn_sacar" => {
                let saque_minimo: i64 = 2000;
                let saldo_atual = crate::banco::ver_saldo(user_id as i64).unwrap_or(0);
                let agora = Utc::now().timestamp();
    
                if saldo_atual >= saque_minimo {
                    let _ = crate::banco::saque(user_id as i64, agora);
                    let texto = format!(
                        "🏦 *Saque Iniciado!*\n
                    Seu saldo: {} 🍎 Frutas\n\n
                    Digite no chat a QUANTIDADE que deseja sacar (Somente números):",
                        saldo_atual
                    );

                    if let Some(c_id) = chat_id {
                        bot.send_message(c_id, texto).await?;
                    }
                    
                } else {
                    let texto = format!(
                        "Saldo insuficiente, o saque minimo é {} 🍎 Frutas",
                        saque_minimo
                    );
                    if let Some(c_id) = chat_id {
                    bot.send_message(c_id, texto).await?;
                    }
                }
            }
            _ => {}
        }
    bot.answer_callback_query(q.id).await?;
}
    Ok(())
}

pub async fn receber_texto(bot: Bot, msg: Message) -> ResponseResult<()> {
    let user_id = msg.from.as_ref().unwrap().id.0;
    let chat_id = msg.chat.id;

    if let Some(texto) = msg.text() {
        if texto.starts_with('/') {
            return Ok(());
        }

        if crate::banco::checar_saque(user_id as i64).unwrap_or(false) {
            let quantidade_salva = crate::banco::ver_quantidade_saque(user_id as i64).unwrap_or(0);

            if quantidade_salva == 0 {
                let qtd_digitada: i64 = match texto.parse() {
                    Ok(numero) => numero,
                    Err(_) => {
                        bot.send_message(
                            chat_id,
                            "❌ Por favor, digite apenas NÚMEROS inteiros para a quantidade.",
                        )
                        .await?;
                        return Ok(());
                    }
                };

                let saldo_atual = crate::banco::ver_saldo(user_id as i64).unwrap_or(0);

                if qtd_digitada > saldo_atual {
                    bot.send_message(
                        chat_id,
                        "❌ Você não tem tudo isso de Frutas! Tente novamente.",
                    )
                    .await?;
                    return Ok(());
                }
                if qtd_digitada < 2000 {
                    bot.send_message(
                        chat_id,
                        "❌ O mínimo de saque é 2000 Frutas. Tente novamente.",
                    )
                    .await?;
                    return Ok(());
                }

                let _ = crate::banco::quantidade_saque(user_id as i64, qtd_digitada);
                bot.send_message(
                    chat_id,
                    "✅ Quantidade confirmada!\n\n
                    Agora, digite seu E-MAIL da FaucetPay:",
                )
                .await?;
                return Ok(());
            } else {
                let email_faucetpay = texto;

                bot.send_message(
                    chat_id,
                    format!(
                        "Processando saque de {} 🍎 Frutas para o e-mail: {}\nAguarde...",
                        quantidade_salva, email_faucetpay
                    ),
                )
                .await?;

                let _ = crate::banco::adicionar_saldo(user_id as i64, -quantidade_salva);

                let valor_dolar: f64 = (quantidade_salva as f64) / 2000.0;

                match crate::payout::pagar(email_faucetpay, valor_dolar).await {
                    Ok(mensagem_sucesso) => {
                        bot.send_message(chat_id, format!("✅ {}", mensagem_sucesso)).await?;
                    }
                    Err(erro) => {
                        let _ = crate::banco::adicionar_saldo(user_id as i64, quantidade_salva);
                        bot.send_message(chat_id, format!("❌ Falha no pagamento: {}\nSuas Frutas foram devolvidas ao seu saldo.", erro)).await?;
                    }
                }
                
                let _ = crate::banco::remover_saque(user_id as i64);
                return Ok(());
            }
        }

        bot.send_message(chat_id, "Comando não reconhecido. Use o menu!")
            .await?;
    }

    Ok(())
}
