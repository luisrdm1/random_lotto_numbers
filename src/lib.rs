use colored::Colorize;
use num_bigint::ToBigUint;
use rand::Rng;
use std::collections::HashSet;
use std::mem::swap;
use std::str::FromStr;
use std::{io, process};

#[derive(Debug, Clone, Copy)]
pub struct Parameters {
    games: usize,
    start: u8,
    end: u8,
    pick: u8,
}

impl Parameters {
    pub fn new(games: usize, start: u8, end: u8, pick: u8) -> Self {
        Parameters {
            games,
            start,
            end,
            pick,
        }
        .validate()
    }

    pub fn validate(&mut self) -> Self {
        if self.end < self.start {
            swap(&mut self.start, &mut self.end);
        }
        let len = self.end + 1 - self.start;
        if len < 2 {
            eprintln!(
                "{}",
                "You need at least two numbers to have some randomness. Aborting."
                    .red()
                    .bold()
            );
            process::exit(1);
        }
        if self.games == 0 {
            eprintln!("{}", "Ok. Aborting.".red().bold());
            process::exit(1);
        }
        if self.pick == 0 {
            eprintln!(
                "{}",
                "Picking zero numbers won't help you! Aborting."
                    .red()
                    .bold()
            );
            process::exit(1);
        }
        if self.pick > len {
            eprintln!(
                "{}",
                "You can't pick more numbers than you have!".bold().red()
            );
            process::exit(1);
        }
        *self
    }

    /// Function that receives the max number and the pick value
    /// to generete random numbers.
    ///
    /// Example:
    /// From 18 numbers, we want to pick 10.
    /// ```
    /// use random_lotto_numbers as rln;
    ///
    /// let param = rln::Parameters::new(1, 1, 18, 10);
    ///
    /// let gen = param.generate_game();
    ///
    /// assert_eq!(10 as usize, gen.len());
    /// ```
    pub fn generate_ticket(&self) -> Vec<u8> {
        let mut game: Vec<u8> = (self.start..=self.end).collect();

        let not_pick = game.len() - self.pick as usize;

        if self.pick as usize >= not_pick {
            let mut count = 0;
            while count < not_pick {
                game.remove(rand::thread_rng().gen_range(0..game.len()));
                count += 1;
            }
        } else {
            game.clear();
            while game.len() != self.pick as usize {
                let random = rand::thread_rng().gen_range(self.start..=self.end);
                if game.contains(&random) {
                    continue;
                } else {
                    game.push(random)
                }
            }
        }
        game.sort();
        game
    }
}

/// Function that receives a user input and expects to make a unsigned/signed integer
/// Uses the FromStr Trait to provide that.
///
pub fn input_into_number<T: FromStr>(string: &str) -> T {
    loop {
        println!("{string}");
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line.");
        match value.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                eprintln!("This is not a valid value. Please try again.");
                continue;
            }
        };
    }
}

pub fn bundle(parameters: Parameters) -> HashSet<Vec<u8>> {
    let mut bundle = HashSet::new();

    let choice_len = parameters.end + 1 - parameters.start;
    let possible_combinations = probability::combinations(choice_len, parameters.pick);

    while bundle.len() != parameters.games {
        let random_ticket = parameters.generate_ticket();

        let inserted = bundle.insert(random_ticket);

        if !inserted
            && bundle.len().to_biguint().unwrap() == possible_combinations
        {
            eprintln!(
                "{}",
                "Unable to generate the requested number of games."
                    .red()
                    .bold()
            );
            break;
        }
    }

    bundle
}

pub mod probability {
    use crate::Parameters;
    use num_bigint::{BigUint, ToBigUint};

    /// N - number of ball in lottery
    /// K - number of balls in a single ticket
    /// B - number of matching balls for a winning ticket
    pub fn odds(parameters: Parameters, b: u8) -> (BigUint, BigUint) {
        let n = parameters.end - parameters.start + 1;
        let k = parameters.pick;
        if k > b {
            return (
                combinations(k, b) * combinations(n - k, k - b),
                combinations(n, k),
            );
        } else {
            return (
                combinations(k, b) * combinations(n - k, k - b),
                combinations(n, k),
            );
        }
    }

    /// Calculates the combinations of (n, r)
    pub fn combinations(n: u8, k: u8) -> BigUint {
        factorial(&n) / (factorial(&k) * factorial(&(n - k)))
    }

    /// Generates the factorial of a number
    fn factorial(n: &u8) -> BigUint {
        match n {
            0 => 1.to_biguint().unwrap(),
            x => {
                let mut x = x.to_biguint().unwrap();
                let n = n.to_owned() as u128;
                for i in 1..n {
                    x *= i;
                }
                x
            }
        }
    }
}
