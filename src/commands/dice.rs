extern crate rand;

use rand::Rng;

// specialties - addtional success / still explodes?

command!(roll(_ctx, msg, args) {

    // TODO: add usage/help
    let dice = args.single::<u8>().unwrap_or(1);
    let difficulty = args.single::<u8>().unwrap_or(6);

    let mut rng = rand::thread_rng();
    let mut roll: Vec<u8> = Vec::new();
    let mut successes = 0;
    let mut tens = 0;

    for _ in 0..dice {
        // extract roll a die maybe?
        // could have a roll d10s thing and just count botches after?
        let die = rng.gen_range(1, 11);
        roll.push(die);

        if die >= difficulty {
            successes += 1;
            // this assumes difficulty is not > 10
            if die == 10 {
                tens += 1;
            }
        }

        if die == 1 {
            successes -= 1;
        }

    }

    let mut tens_string = String::new();
    if tens > 0 {
        // TODO:
        // call tens roll function to get tens rolls
        // initial call: roll_more_tens_maybe(difficulty, successes, vec![(tens, roll)], rng);
        // use tens rolls to append to tens_string
    }

    if let Err(why) = msg.channel_id.say(
        // TODO: name these & make it readable
        format!("{} rolled {} {} at difficulty {}\nRoll: {:?}\n{}Successes: {}",
                msg.author.name,
                dice,
                if dice == 1 { "die" } else { "dice" },
                difficulty,
                roll,
                tens_string,
                successes)
    ) {
        println!("Error sending {}'s roll: {:?} : {}", msg.author.name, roll, why);
    };

});

// TODO: do none of the string shit in that function
// do all the recursive rolling, returning a (u8, Vec<(u8 Vec<u8>)>)
// total successes, vec of tuples of number of tens in each roll and the roll
// then iterate over the vec vec to build the string
// is it faster to just count the tens again instead of allocating? maybe/probably

// // initial call: roll_more_tens_maybe(difficulty, successes, vec![(tens, roll)], rng);
// // returns (successes, Vec<(tens_in_roll, roll)>)
// fn roll_more_tens_maybe(
//     difficulty: u8,
//     mut successes: u8,
//     mut rolls: Vec<(u8, Vec<u8>)>,
//     mut rng: rand::ThreadRng,
// ) -> (u8, Vec<(u8, Vec<u8>)>) {
//     // this should never be called with empty rolls
//     let tens = rolls.last().unwrap().0;
//     let mut roll = Vec::new();
//     let mut new_tens = 0;
//     for _ in 0..tens {
//         let die = rng.gen_range(1, 11);
//         roll.push(die);

//         if die >= difficulty {
//             successes += 1;
//             // this assumes difficulty is not > 10
//             if die == 10 {
//                 new_tens += 1;
//             }
//         }
//     }
//     rolls.push((new_tens, roll));

//     // recursively call maybe
//     if new_tens > 0 {
//         roll_more_tens_maybe(difficulty, successes, rolls, rng)
//     } else {
//         (successes, rolls)
//     }

// }

// DEPRECATED
// writes to msg string and returns final successes count
fn append_tens_roll(
    tens: u8,
    difficulty: u8,
    mut successes: u8,
    msg: &mut String,
    mut rng: rand::ThreadRng,
) -> (u8, &mut String) {
    let omg = format!("\n{} tens rolled!", tens);
    msg.push_str(&omg);

    // roll dice copypasta
    let mut tens = 0;
    let mut roll: Vec<u8> = Vec::new();
    for _ in 0..tens {
        let die = rng.gen_range(1, 11);
        roll.push(die);

        if die >= difficulty {
            successes += 1;
            // this assumes difficulty is not > 10
            if die == 10 {
                tens += 1;
            }
        }
    }

    // let rerolls = format!("\nRerolls: {:?}", roll);
    // msg.push_str(&rerolls);

    let rerolls = format!("\nRerolls: {:?}", roll);
    msg.push_str(&rerolls);

    // if more tens are rolled, recurse
    if tens > 0 {
        // TODO: why does it compile w/o &mut msg?
        append_tens_roll(tens, difficulty, successes, msg, rng);
    }

    (successes, msg)
}
