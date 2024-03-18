use poise::serenity_prelude::{Context, FullEvent};

use crate::{Data, Error};

mod message_event;
mod interaction_event;

pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        FullEvent::Message { new_message } => {
            message_event::message(ctx, new_message.clone(), &data.db).await;
        }
        FullEvent::InteractionCreate { interaction } => {
            interaction_event::interaction_create(ctx, interaction.clone(), &data.db).await;
        }
        _ => {}
    }
    Ok(())
}