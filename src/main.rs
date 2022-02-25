use anyhow::{Context, Result};
use codify_bot::commands::*;
use codify_bot::state::State;
use codify_bot::services;
use poise::serenity_prelude::GatewayIntents;
use poise::{Framework, FrameworkOptions};
use std::env;
#[tokio::main]
async fn main() -> Result<()> {
    services::init()?;
    dotenv::dotenv().with_context(|| format!("Can't load .env"))?;
    Framework::build()
        .token(env::var("DISCORD_MAIN_INSTANCE_TOKEN").unwrap())
        .user_data_setup(|_, _, _| Box::pin(async move { Ok(State {}) }))
        .options(FrameworkOptions {
            on_error: |err| {
                Box::pin(async move {
                    tracing::error!("{err:?}");
                })
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                )),
                case_insensitive_commands: true,
                additional_prefixes: vec![
                    poise::Prefix::Literal("codify"),
                    poise::Prefix::Literal("codify,"),
                 ],
                ..Default::default()
            },

            commands: vec![play(), help(), atualizar(), report()],
            ..Default::default()
        })
        .client_settings(|client| {
            client.intents(
                GatewayIntents::non_privileged()
                    | GatewayIntents::GUILDS
                    | GatewayIntents::GUILD_MESSAGES,
            )
        })
        .run()
        .await?;
    Ok(())
}
