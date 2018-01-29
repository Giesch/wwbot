extern crate rand;

use std::{thread, time};

use serenity::utils::MessageBuilder;

use commands::dice::*;

fn read_dice(config: &mut Config, opt: &str) {
    if let Ok(num) = opt.parse::<u8>() {
        config.dice = num;
    }
}

// TODO: i only thought i understood what mut means
fn read_diff(mut config: &mut Config, opt: &str) {
    match opt {
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            config.difficulty = opt.parse::<u8>().unwrap();
        }
        _ => {
            read_option(&mut config, opt);
        }
    }
}

fn read_option(config: &mut Config, opt: &str) {
    match opt {
        "special" | "specialty" | "s" | "-s" => {
            config.specialty = true;
        }
        _ => (),
    }
}

command!(roll(_ctx, msg, args) {
    let mut config = Config {
        dice: 1,
        difficulty: 6,
        specialty: false
    };

    if let Ok(string) = args.single::<String>() {
        read_dice(&mut config, &string);
    }
    if let Ok(string) = args.single::<String>() {
        read_diff(&mut config, &string);
    }
    if let Ok(string) = args.single::<String>() {
        read_option(&mut config, &string);
    }

    let mut rng = rand::thread_rng();

    let initial_roll = initial_roll(&mut rng, &mut config);
    let tens_rolls = tens_rolls(&config, &initial_roll);

    let r = format!("{:?}", initial_roll.dice);

    let mut message_builder = MessageBuilder::new()
        .user(msg.author.id)
        .push(" rolled ").push(config.dice)
        .push(if config.dice == 1 { " die" } else { " dice" })
        .push(" at difficulty ").push(config.difficulty)
        .push("\nRoll: ").push(r)
        .push("\nSuccesses: ").push(initial_roll.successes);

    let message_builder = if initial_roll.tens > 0 {
        message_builder.push(
            format!("\n{} {} rolled!",
                    initial_roll.tens,
                    if initial_roll.tens > 1 { "tens" } else { "ten" })
        )
    } else {
        message_builder
    };

    let response = message_builder.build();

    if let Err(why) = msg.channel_id.say(response) {
        println!("Error sending {}'s roll: {:?} : {}", msg.author.name, initial_roll.dice, why);
    };

    for tens_roll in tens_rolls.rolls {
        thread::sleep(time::Duration::from_millis(1500));
        let r = format!("{:?}", tens_roll.dice);

        let message_builder = MessageBuilder::new().push(r);

        let message_builder = if tens_roll.tens > 0 {
            message_builder.push(
                format!("\n{} more {}", tens_roll.tens,
                        if tens_roll.tens > 1 { "tens..." } else { "ten..." })
            )
        } else {
            message_builder
        };

        let message_builder = if tens_roll.last {
            message_builder.push("\n").user(msg.author.id).push(format!(" got {} successes", tens_rolls.successes))
        } else {
            message_builder
        };

        let response = message_builder.build();

        if let Err(why) = msg.channel_id.say(response) {
            println!("Error sending {}'s tens roll: {:?} : {}", msg.author.name, initial_roll, why);
        };
    }
});
