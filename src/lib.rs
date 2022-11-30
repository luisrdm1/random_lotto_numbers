use rand::Rng;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
pub struct Parameters {
    pub games: u16,
    numbers_start: u8,
    numbers_end: u8,
    pick: u8,
}

impl Parameters {
    ///
    pub fn new(g: u16, ns: u8, ne: u8, pick: u8) -> Self {
        Parameters {
            games: g,
            numbers_start: ns,
            numbers_end: ne,
            pick,
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
        let mut games: Vec<u8> = (self.numbers_start..=self.numbers_end).collect();

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
                let random = rand::thread_rng().gen_range(self.numbers_start..=self.numbers_end);
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
