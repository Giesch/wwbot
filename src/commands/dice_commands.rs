extern crate rand;

use std::fmt;
use std::{thread, time};

use rand::Rng;
use rand::ThreadRng;
use serenity::utils::MessageBuilder;

use commands::dice::{TensRoll, TensRolls};

pub struct Config {
    pub dice: u8,
    pub difficulty: u8,
    pub specialty: bool,
}

impl Config {
    fn read_dice(&mut self, opt: &str) {
        if let Ok(num) = opt.parse::<u8>() {
            self.dice = num;
        }
    }

    fn read_diff(&mut self, opt: &str) {
        match opt {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                self.difficulty = opt.parse::<u8>().unwrap();
            }
            _ => {
                self.read_option(opt);
            }
        }
    }

    fn read_option(&mut self, opt: &str) {
        match opt {
            "special" | "specialty" | "s" | "-s" => {
                self.specialty = true;
            }
            _ => (),
        }
    }
}

command!(roll(_ctx, msg, args) {
    let mut config = Config {
        dice: 1,
        difficulty: 6,
        specialty: false
    };

    if let Ok(string) = args.single::<String>() {
        config.read_dice(&string);
    }
    if let Ok(string) = args.single::<String>() {
        config.read_diff(&string);
    }
    if let Ok(string) = args.single::<String>() {
        config.read_option(&string);
    }

    let mut rng = rand::thread_rng();
    let mut roll: Vec<u8> = Vec::new();
    let mut successes: i32 = 0;
    let mut tens = 0;

    for _ in 0..config.dice {
        let die = rng.gen_range(1, 11);
        roll.push(die);

        if die >= config.difficulty {
            successes += 1;
            if die == 10 {
                tens += 1;
                if config.specialty { successes += 1; }
            }
        }

        if die == 1 {
            successes -= 1;
        }
    }

    let mut tens_rolls = TensRolls {
        rolls: vec![],
        difficulty: config.difficulty,
        specialty: config.specialty,
        successes,
        rng,
    };

    let mut tens_string = String::new();
    if tens > 0 {
        tens_string = format!("\n{} {} rolled!", tens, if tens > 1 { "tens" } else { "ten" });
        tens_rolls.roll_more_tens_maybe(tens);
    }

    let r = format!("{:?}", roll);
    let response = MessageBuilder::new()
        .user(msg.author.id)
        .push(" rolled ").push(config.dice)
        .push(if config.dice == 1 { " die" } else { " dice" })
        .push(" at difficulty ").push(config.difficulty)
        .push("\nRoll: ").push(r)
        .push("\nSuccesses: ").push(successes)
        .push(&tens_string)
        .build();

    if let Err(why) = msg.channel_id.say(response) {
        println!("Error sending {}'s roll: {:?} : {}", msg.author.name, roll, why);
    };

    for tens_roll in tens_rolls.rolls {
        thread::sleep(time::Duration::from_millis(1500));
        let r = format!("{:?}", tens_roll.roll);

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
            println!("Error sending {}'s tens roll: {:?} : {}", msg.author.name, roll, why);
        };
    }
});