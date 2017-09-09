extern crate rand;

use rand::Rng;

command!(roll(_ctx, msg, args) {

    // TODO: add usage/help
    let dice = args.single::<u8>().unwrap_or(1);
    let difficulty = args.single::<u8>().unwrap_or(6);

    let mut rng = rand::thread_rng();
    let mut roll: Vec<u8> = Vec::new();
    let mut successes = 0;
    for _ in 0..dice {
        let die = rng.gen_range(1, 11);
        roll.push(die);

        if die >= difficulty {
            successes += 1;
        }

        if die == 1 {
            successes -= 1;
        }
    }

    if let Err(why) =
        msg.channel_id.say(format!("{} rolled {} {} at difficulty {}\nRoll: {:?}\nSuccesses: {}",
                                   msg.author.name,
                                   dice,
                                   if dice == 1 { "die" } else { "dice" },
                                   difficulty, roll, successes)) {

            println!("Error sending {}'s roll: {:?} : {}", msg.author.name, roll, why);
        };

});
