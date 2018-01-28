#[macro_use]
extern crate serenity;
extern crate rand;

mod commands;

use serenity::framework::StandardFramework;
use serenity::prelude::*;
use serenity::model::*;
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap(), Handler);

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~"))
            .command("ping", |c| c.exec(commands::meta::ping))
            .command("latency", |c| c.exec(commands::meta::latency))
            .command("roll", |c| {
                c.known_as("r").exec(commands::dice_commands::roll)
            }),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
