use colored::Colorize;
use rand::Rng;
use std::str::FromStr;
use std::{io, process};

#[derive(Debug)]
pub struct Parameters {
    pub games: usize,
    start: u8,
    end: u8,
    pick: u8,
}

impl Parameters {
    ///
    pub fn new(games: usize, start: u8, end: u8, pick: u8) -> Self {
        Parameters {
            games,
            start,
            end,
            pick,
        }
    }

    pub fn validate(&self) {
        if let 0 = self.games {
            eprintln!("{}", "Ok. Aborting.".red().bold());
            process::exit(1);
        }
        if let true = self.end <= self.start {
            eprintln!("{}", "We need at least two numbers. Aborting.".bold().red());
            process::exit(1);
        }
        if let 0 = self.pick {
            eprintln!(
                "{}",
                "Picking zero numbers won't help you. Aborting."
                    .red()
                    .bold()
            );
            process::exit(1);
        }
        if self.pick > (self.end - self.start) {
            eprintln!(
                "{}",
                "You can't pick more numbers than you have!".bold().red()
            );
            process::exit(1);
        }
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
    pub fn generate_game(&self) -> Vec<u8> {
        let mut games: Vec<u8> = (self.start..=self.end).collect();

        let not_pick = games.len() - self.pick as usize;

        if self.pick as usize >= not_pick {
            let mut count = 0;
            while count < not_pick {
                games.remove(rand::thread_rng().gen_range(0..games.len()));
                count += 1;
            }
        } else {
            games.clear();
            while games.len() < self.pick as usize {
                let random = rand::thread_rng().gen_range(self.start..=self.end);
                if games.contains(&random) {
                    continue;
                } else {
                    games.push(random)
                }
            }
        }
        games.sort();
        games
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

pub fn run(parameters: Parameters) {
    loop {
        let mut count = 0;

        while count < parameters.games {
            let random_numbers = Parameters::generate_game(&parameters);

            for num in random_numbers {
                let color_num = num.to_string().bright_green();
                if num < 10 {
                    let zero = 0.to_string().bright_green();
                    print!("{zero}{color_num} ");
                } else {
                    print!("{color_num} ");
                }
            }
            println!();
            count += 1;
        }
        break;
    }
}

pub mod probability {
    use num_bigint::{BigUint, ToBigUint};

    /// Calculates Permutation of (n, r) as
    ///
    ///
    pub fn combinations(n: u8, r: u8) -> BigUint {
        factorial(&n) / (factorial(&r) * factorial(&(n - r)))
    }

    /// Generates the factorial of a number
    fn factorial(n: &u8) -> BigUint {
        match n {
            0 => 0.to_biguint().unwrap(),
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
