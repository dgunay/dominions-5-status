use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::{builder::CreateEmbed, CacheAndHttp};

use crate::db::*;

use crate::model::{GameServer, GameServerState};

pub fn lobbies(context: &mut Context, message: &Message) -> Result<(), CommandError> {
    let data = context.data.read();
    let db_conn = data
        .get::<DbConnectionKey>()
        .ok_or_else(|| CommandError("No db connection".to_string()))?;

    let lobbies_and_player_count = db_conn.select_lobbies()?;
    if lobbies_and_player_count.is_empty() {
        message.reply(CacheAndHttp::default(), &"No available lobbies")?;
    } else {
        // let embed = lobbies_helper(lobbies_and_player_count)?;
        message.channel_id.send_message(&context.http, |m| {
            m.embed(|m| lobbies_helper2(lobbies_and_player_count, m))
        })?;
    }
    Ok(())
}

// fn lobbies_helper(
//     lobbies_and_player_count: Vec<(GameServer, i32)>,
// ) -> Result<CreateEmbed, CommandError> {
//     let mut aliases = String::new();
//     let mut player_counts = String::new();

//     for (lobby, registered_count) in lobbies_and_player_count {
//         aliases.push_str(&format!("{}\n", lobby.alias));
//         if let GameServerState::Lobby(state) = lobby.state {
//             player_counts.push_str(&format!("{}/{}\n", registered_count, state.player_count));
//         } else {
//             player_counts.push_str(&"ERROR");
//         }
//     }

//     let embed = CreateEmbed::default()
//         .title("Lobbies")
//         .field("Alias", aliases, true)
//         .field("Players", player_counts, true);

//     Ok(*embed)
// }

// Attempt at something that will work with the new &mut API to embeds. Works
// more like a setter/builder method.
fn lobbies_helper2(
    lobbies_and_player_count: Vec<(GameServer, i32)>,
    embed: &mut CreateEmbed,
) -> &mut CreateEmbed {
    let mut aliases = String::new();
    let mut player_counts = String::new();

    for (lobby, registered_count) in lobbies_and_player_count {
        aliases.push_str(&format!("{}\n", lobby.alias));
        if let GameServerState::Lobby(state) = lobby.state {
            player_counts.push_str(&format!("{}/{}\n", registered_count, state.player_count));
        } else {
            player_counts.push_str(&"ERROR");
        }
    }

    embed
        .title("Lobbies")
        .field("Alias", aliases, true)
        .field("Players", player_counts, true)
}
