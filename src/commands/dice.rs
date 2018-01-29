extern crate rand;

use std::fmt;
use std::{thread, time};

use rand::Rng;
use rand::ThreadRng;
use serenity::utils::MessageBuilder;

use commands::dice_commands::Config;

pub fn intitial_roll(mut rng: ThreadRng, config: Config, mut roll: Vec<u8>) -> TensRolls {
    let mut successes: i32 = 0;
    let mut tens = 0;

    for _ in 0..config.dice {
        let die = rng.gen_range(1, 11);
        roll.push(die);

        if die >= config.difficulty {
            successes += 1;
            if die == 10 {
                tens += 1;
                if config.specialty {
                    successes += 1;
                }
            }
        }

        if die == 1 {
            successes -= 1;
        }
    }

    TensRolls {
        rolls: vec![],
        difficulty: config.difficulty,
        specialty: config.specialty,
        successes,
        rng,
    }
}


pub struct TensRoll {
    pub last: bool,
    pub tens: u8,
    pub roll: Vec<u8>,
}

impl fmt::Display for TensRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.roll)
    }
}

pub struct TensRolls {
    pub successes: i32,
    pub rolls: Vec<TensRoll>,
    pub rng: ThreadRng,
    pub difficulty: u8,
    pub specialty: bool,
}

impl TensRolls {
    pub fn roll_more_tens_maybe(&mut self, dice_to_roll: u8) {
        if dice_to_roll == 0 {
            return;
        }

        let mut roll: Vec<u8> = Vec::new();
        let mut successes: i32 = 0;
        let mut tens = 0;

        for _ in 0..dice_to_roll {
            let die = self.rng.gen_range(1, 11);
            roll.push(die);

            if die >= self.difficulty {
                successes += 1;
            }

            if die >= 10 {
                tens += 1;
                if self.specialty {
                    successes += 1;
                }
            }
        }

        self.successes += successes;
        self.rolls.push(TensRoll {
            tens,
            roll,
            last: tens == 0,
        });

        self.roll_more_tens_maybe(tens);
    }
}
