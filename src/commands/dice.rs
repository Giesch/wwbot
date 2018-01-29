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
    pub dice: Vec<u8>,
    pub successes: i32,
    pub tens: u8,
    pub ones: u8,
}

fn roll_d10s(rng: &mut ThreadRng, n: u8) -> Vec<u8> {
    let mut dice: Vec<u8> = Vec::new();
    for _ in 0..n {
        let die = rng.gen_range(1, 11);
        dice.push(die);
    }

    dice
}

fn count_roll(dice: Vec<u8>, difficulty: u8, specialty: bool) -> Roll {
    let mut successes = 0;
    let mut tens = 0;
    let mut ones = 0;

    for &die in dice.iter() {
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
        dice,
        successes,
        tens,
        ones,
    }
}

pub fn initial_roll(rng: &mut ThreadRng, config: &Config) -> Roll {
    let dice = roll_d10s(rng, config.dice);
    let mut roll = count_roll(dice, config.difficulty, config.specialty);
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
    pub dice: Vec<u8>,
}

impl fmt::Display for TensRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.dice)
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

        let dice = roll_d10s(&mut self.rng, dice_to_roll);
        let roll = count_roll(dice, self.difficulty, self.specialty);

        self.successes += roll.successes;
        self.rolls.push(TensRoll {
            tens: roll.tens,
            dice: roll.dice,
            last: roll.tens == 0,
        });

        self.roll_more_tens_maybe(roll.tens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_roll_counts_successes() {
        let dice = vec![1, 2, 3, 4];
        let roll = count_roll(dice, 3, false);
        assert_eq!(2, roll.successes);
    }

    #[test]
    fn count_roll_counts_ones() {
        let dice = vec![1, 2, 3, 4];
        let roll = count_roll(dice, 3, false);
        assert_eq!(1, roll.ones);
    }

    #[test]
    fn count_roll_counts_tens() {
        let dice = vec![1, 2, 10, 10];
        let roll = count_roll(dice, 3, false);
        assert_eq!(2, roll.tens);
    }

    #[test]
    fn count_roll_doubles_tens_for_specialties() {
        let dice = vec![1, 2, 10, 10];
        let roll = count_roll(dice, 3, true);
        assert_eq!(4, roll.successes);
    }
}
