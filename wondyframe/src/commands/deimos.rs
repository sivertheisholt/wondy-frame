use std::{sync::{Arc, OnceLock}, time::{Duration, Instant}};
use tokio::sync::Mutex;
use serenity::all::CreateEmbed;
use serenity::all::CreateEmbedFooter;
use tracing::{error, info};
use warframe::worldstate::{TimedEvent, queryable::CambionDrift};

use crate::enums::colors::Colors;
use crate::enums::thumbnail::Thumbnail;
use crate::types::context::Context;
use crate::types::error::Error;
use crate::utils::date::format_timestamp;

#[poise::command(
    slash_command,
    description_localized("en-US", "Get current Deimos details")
)]
pub async fn deimos(
    ctx: Context<'_>,
    #[description = "Reply visible to other users?"] public: Option<bool>,
) -> Result<(), Error> {
    info!("Deimos command called");
    let is_public = public.unwrap_or(false);
    let client: &warframe::worldstate::Client = &ctx.data().client;
    // One shared cache per world-state endpoint, including concurrent requests.
    static CACHE: OnceLock<Mutex<Option<(Instant, Arc<CambionDrift>)>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(None));
    let mut cached = cache.lock().await;
    let fresh = cached.as_ref().and_then(|(created, value)| {
        (created.elapsed() < Duration::from_secs(300)).then(|| Arc::clone(value))
    });
    let cambion_drift: Arc<CambionDrift> = if let Some(value) = fresh {
        value
    } else {
        match client.fetch::<CambionDrift>().await {
            Ok(value) => {
                let value = Arc::new(value);
                *cached = Some((Instant::now(), Arc::clone(&value)));
                value
            }
            Err(e) => {
                error!("Error fetching data from API: {:?}", e);
                ctx.send(
                    poise::CreateReply::default()
                        .content("Could not fetch Deimos data at this time due to external failure with Warframe API.")
                        .ephemeral(!is_public),
                )
                .await?;
                return Ok(());
            }
        }
    };
    drop(cached);

    let embed: CreateEmbed = CreateEmbed::new()
        .title("Deimos")
        .url("https://warframe.fandom.com/wiki/Deimos")
        .color(Colors::EmbedColor.as_u32())
        .field("Current state", cambion_drift.state.to_string(), false)
        .field("Time left", cambion_drift.eta().to_string(), false)
        .thumbnail(Thumbnail::Deimos.url())
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
