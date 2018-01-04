use serenity::framework::standard::{Args, CommandError};
use serenity::prelude::Context;
use serenity::model::Message;

use server::get_game_data;
use model::player::Player;
use db::DbConnectionKey;

pub fn pm_players(context: &mut Context, message: &Message, mut args: Args) -> Result<(), CommandError> {
    // println!("pm_players");
    // let alias = args.single::<String>().or_else(|_| {
    //     message.channel_id.name().ok_or(format!("Could not find channel name for channel {}", message.channel_id))
    // })?;

    // let data = context.data.lock();
    // let server_list = data.get::<ServerList>().ok_or("No ServerList was created on startup. This is a bug.")?;
    // let server = server_list.get(&alias).ok_or(format!("Could not find server {}", alias))?;
    
    // println!("ready to start pming players");
    // for (user_id, player) in &server.players {
    //     let text = format!("Hi, you are {} in {}", player.nation_name, alias);
    //     println!("telling {} {}", user_id, text);
    //     let private_channel = user_id.create_dm_channel()?;
    //     private_channel.say(&text)?;
    // }
    // println!("finished PMing players");
    Ok(())

}

pub fn show_registered(context: &mut Context, message: &Message, mut args: Args) -> Result<(), CommandError> {
    // println!("show_registered");
    // let alias = args.single::<String>().or_else(|_| {
    //     message.channel_id.name().ok_or(format!("Could not find channel name for channel {}", message.channel_id))
    // })?;

    // let data = context.data.lock();
    // let server_list = data.get::<ServerList>().ok_or("No ServerList was created on startup. This is a bug.")?;
    // let server = server_list.get(&alias).ok_or(format!("Could not find server {}", alias))?;
    // let text = format!("server {} has players {:?}", alias, server.players);
    // println!("replying with {}", text);
    // let _ = message.reply(&text);
    Ok(())
}

pub fn unregister_player(context: &mut Context, message: &Message, mut args: Args) -> Result<(), CommandError> {
    // println!("unregistering player");
    // let alias = args.single::<String>().or_else(|_| {
    //     message.channel_id.name().ok_or(format!("Could not find channel name for channel {}", message.channel_id))
    // })?;

    // let mut data = context.data.lock();
    // let server_list = data.get_mut::<ServerList>().ok_or("No ServerList was created on startup. This is a bug.")?;

    // let server = server_list.get_mut(&alias).ok_or(format!("Could not find server {}", alias))?;
    // let ref user = message.author;
    // let _ = server.players.remove(&user.id);
    // let text = format!("Removing user {} from game {}", user.name, alias);
    // println!("{}", text);
    // let _ = message.reply(&text);
    Ok(())
}

pub fn register_player(context: &mut Context, message: &Message, mut args: Args) -> Result<(), CommandError> {
    println!("registering player");
    let arg_nation_name = args.single::<String>()?.to_lowercase();   
    let alias = args.single::<String>().or_else(|_| {
        message.channel_id.name().ok_or("")
    })?;

    let mut data = context.data.lock();
    let db_conn = data.get::<DbConnectionKey>().unwrap();
    let server = db_conn.game_for_alias(alias.clone()).unwrap();
    // let server_address = server.address.clone();
    let data = get_game_data(&server.address)?;

    let mut nation_names = data.nations.iter().map(|nation| &nation.name);
    let nation_name = nation_names.find(|&name| // TODO: more efficient algo
        name.to_lowercase().starts_with(&arg_nation_name) 
    ).ok_or_else(|| {
        let err = format!("Could not find nation starting with {}", arg_nation_name);
        println!("{}", err);
        err
    })?; 

    // FIXME: currently assumes that the player does not exist
    let player = Player {
        discord_user_id: message.author.id,
    }; 

    // TODO: transaction
    db_conn.insert_player(&player).unwrap();
    // FIXME: nation number
    db_conn.insert_server_player(&server.address.clone(), &message.author.id, 0).unwrap();

    // let text = format!("registering nation {} for user {} in game {}", nation_name, user_name, data.game_name);
    
    // TODO:
        // look for player with discord_user_id
        // if does not exist, create
        // insert player_server record
    
    

    // let _ = server.players.insert(user.id, player);
    // let _ = message.reply(&text);
    Ok(())
}
