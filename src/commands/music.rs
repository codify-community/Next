use crate::Context;
use anyhow::Result;
#[poise::command(prefix_command, slash_command)]
/// Toca uma música (PARA FAZER)
pub async fn play(cx: Context<'_>) -> Result<()> {
    cx.defer_or_broadcast().await?;
    cx.say("Hello, world!").await?;
    Ok(())
}