pub mod commands;
use state::State;

pub mod state;
pub mod services;
pub type Data = State;
pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;