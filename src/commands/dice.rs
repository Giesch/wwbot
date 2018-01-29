extern crate rand;

use std::fmt;

use rand::Rng;
use rand::ThreadRng;

pub struct Config {
    pub dice: u8,
    pub difficulty: u8,
    pub specialty: bool,
}

#[derive(Debug)]
pub struct Roll {
    pub roll: Vec<u8>,
    pub successes: i32,
    pub tens: u8,
    pub ones: u8,
}

fn roll_d10s(rng: &mut ThreadRng, dice: u8, difficulty: u8, specialty: bool) -> Roll {
    let mut roll: Vec<u8> = Vec::new();
    let mut successes = 0;
    let mut tens = 0;
    let mut ones = 0;

    for _ in 0..dice {
        let die = rng.gen_range(1, 11);
        roll.push(die);

        if die >= difficulty {
            successes += 1;
            if die == 10 {
                tens += 1;
                if specialty {
                    successes += 1;
                }
            }
        }
        if die == 1 {
            ones += 1;
        }
    }

    Roll {
        roll,
        successes,
        tens,
        ones,
    }
}

pub fn initial_roll(rng: &mut ThreadRng, config: &mut Config) -> Roll {
    let mut roll = roll_d10s(rng, config.dice, config.difficulty, config.specialty);
    roll.successes -= roll.ones as i32;

    roll
}

pub fn tens_rolls(config: &Config, initial_roll: &Roll) -> TensRolls {
    let mut tens_rolls = TensRolls {
        rolls: vec![],
        difficulty: config.difficulty,
        specialty: config.specialty,
        successes: initial_roll.successes,
        rng: rand::thread_rng(),
    };
    tens_rolls.roll_more_tens_maybe(initial_roll.tens);

    tens_rolls
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

        let roll = roll_d10s(&mut self.rng, dice_to_roll, self.difficulty, self.specialty);

        self.successes += roll.successes;
        self.rolls.push(TensRoll {
            tens: roll.tens,
            roll: roll.roll,
            last: roll.tens == 0,
        });

        self.roll_more_tens_maybe(roll.tens);
    }
}
