use std::{sync::{Arc, OnceLock}, time::{Duration, Instant}};
use tokio::sync::Mutex;
use serenity::all::CreateEmbed;
use serenity::all::CreateEmbedFooter;
use tracing::{error, info};
use warframe::worldstate::{TimedEvent, queryable::Cetus};

use crate::enums::colors::Colors;
use crate::enums::thumbnail::Thumbnail;
use crate::types::context::Context;
use crate::types::error::Error;
use crate::utils::date::format_timestamp;

#[poise::command(
    slash_command,
    description_localized("en-US", "Get current Cetus details")
)]
pub async fn cetus(
    ctx: Context<'_>,
    #[description = "Reply visible to other users?"] public: Option<bool>,
) -> Result<(), Error> {
    info!("Cetus command called");
    let is_public = public.unwrap_or(false);
    let client: &warframe::worldstate::Client = &ctx.data().client;
    // One shared cache per world-state endpoint, including concurrent requests.
    static CACHE: OnceLock<Mutex<Option<(Instant, Arc<Cetus>)>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(None));
    let mut cached = cache.lock().await;
    let fresh = cached.as_ref().and_then(|(created, value)| {
        (created.elapsed() < Duration::from_secs(300)).then(|| Arc::clone(value))
    });
    let cetus: Arc<Cetus> = if let Some(value) = fresh {
        value
    } else {
        match client.fetch::<Cetus>().await {
            Ok(value) => {
                let value = Arc::new(value);
                *cached = Some((Instant::now(), Arc::clone(&value)));
                value
            }
            Err(e) => {
                error!("Error fetching data from API: {:?}", e);
                ctx.send(
                    poise::CreateReply::default()
                        .content("Could not fetch Cetus data at this time due to external failure with Warframe API.")
                        .ephemeral(!is_public),
                )
                .await?;
                return Ok(());
            }
        }
    };
    drop(cached);

    let embed: CreateEmbed = CreateEmbed::new()
        .title("Cetus")
        .url("https://warframe.fandom.com/wiki/Cetus")
        .color(Colors::EmbedColor.as_u32())
        .field("Current state", cetus.state.to_string(), false)
        .field("Time left", cetus.eta().to_string(), false)
        .thumbnail(Thumbnail::Cetus.url())
        .footer(CreateEmbedFooter::new(format!(
            "World state updated: {}",
            format_timestamp()
        )));

    let reply = poise::CreateReply::default()
        .embed(embed)
        .ephemeral(!is_public);

    ctx.send(reply).await?;

    Ok(())
}
