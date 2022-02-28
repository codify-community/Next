use crate::Context;
use anyhow::Result;
use poise::serenity_prelude::{self as serenity, ChannelId, Mentionable};
use std::env;
#[poise::command(context_menu_command = "Reportar Mensagem", slash_command)]
/// Reporta a mensagem para um moderador do servidor.
pub async fn report(
    cx: Context<'_>,
    #[description = "Mensagem para reportar"] msg: serenity::Message,
) -> Result<()> {
    let guild = cx
        .guild_id()
        .ok_or(anyhow::anyhow!("wtf, isso não era para acontecer..."))?;
    let channels = guild.channels(cx.discord()).await?;
    let (_, channel) = channels.iter().find(|(id, ..)| {
        **id == ChannelId(env::var("CODIFY_REPORT_CHANNEL").unwrap().parse().unwrap())
    }).ok_or(anyhow::anyhow!("wtf, eu não encontrei o canal para reportar, você tem certeza que configurou corretamente?"))?;
    let report_channel = msg.channel(cx.discord()).await?;
    channel
        .send_message(cx.discord(), |m| {
            m.add_files(msg.attachments.iter().map(|a| a.url.as_str()))
                .add_embed(|e| {
                    e.title("Novo Report!")
                        .field("Mensagem", format!("```md\n{}\n```", msg.content), true)
                        .field("Autor", msg.author.mention(), true)
                        .field("Canal", report_channel.mention(), false)
                        .colour(cx.author().accent_colour.unwrap_or_default())
                        .author(|f| {
                            f.name(&cx.author().name).icon_url(
                                cx.author().avatar_url().as_ref().unwrap_or(&String::new()),
                            )
                        })
                })
        })
        .await?;
    tracing::info!("👀 Reportado: {}@{}", msg.content, msg.author);
    cx.send(|m| m.content("Reportado! Muito Obrigado!")).await?;
    Ok(())
}
