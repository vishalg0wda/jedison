mod mistral;

use crate::mistral::MistralSDK;
use dotenvy::dotenv;
use eyre::WrapErr;
use mistral::MistralAuth;
use owo_colors::OwoColorize;
use std::env;

fn main() -> eyre::Result<()> {
    dotenv().wrap_err("failed to infer env")?;
    let auth =
        MistralAuth::new(env::var("MISTRAL_AI_API_KEY").wrap_err("Please supply MISTRAL_API_KEY")?);
    let agent = env::var("MISTRAL_AI_AGENT_ID").wrap_err("Please supply MISTRAL_AI_AGENT_ID")?;
    let mistral = MistralSDK::new(auth).with_default_agent(agent);
    let message = "yowza?!";
    println!("{:>10}: \"{}\"", "You".bright_white(), message.white());
    let reply = mistral.chat(message)?;
    println!("{:>10}: \"{}\"", "Snarko".bright_yellow(), reply.yellow());
    Ok(())
}
