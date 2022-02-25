use crate::Context;
use anyhow::Result;
#[poise::command(prefix_command, track_edits, slash_command, aliases("comandos", "ajuda"))]
/// Mostra os comandos
pub async fn help(
    ctx: Context<'_>,
    #[description = "Comando para mostrar a descrição"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<()> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Codify Comunnity 2022",
            show_context_menu_commands: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
#[poise::command(prefix_command, hide_in_help)]
pub async fn atualizar(ctx: Context<'_>) -> Result<()> {
    poise::builtins::register_application_commands(ctx, false).await?;
    ctx.say("Comandos atualizados :thumbsup:").await?;
    Ok(())
}
