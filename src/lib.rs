use rand::Rng;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
pub struct Parameters {
    games: u16,
    numbers_start: u8,
    numbers_end: u8,
    pick: u8,
}

impl Parameters {
    fn new() -> Self {
        Parameters {
            games: 1,
            numbers_start: 1,
            numbers_end: 10,
            pick: 5
        }
    }

    fn () {
        
    }
}

/// Function that receives a user input and expects to make a number
/// FromStr.
/// 
/// Examples:
/// From 18 numbers, we want to pick 10.
/// ```
/// use random_lotto_numbers as rln;
/// 
/// let input = rln::input_into_number::<u8>(&String::from(" 7".trim()));
/// 
/// assert_eq!(7 as u8, input);
/// ```
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


/// Function that receives the max number and the pick value
/// to generete random numbers.
/// 
/// Example:
/// From 18 numbers, we want to pick 10.
/// ```
/// use random_lotto_numbers as rln;
/// 
/// let x: u8 = 18;
/// let y: u8 = 10;
/// 
/// let gen = rln::generate_numbers(x, y);
/// 
/// assert_eq!(10 as usize, gen.len());
/// ```
pub fn generate_numbers(numbers: u8, pick: u8) -> Vec<u8> {
    let mut game_numbers: Vec<u8> = (1..=numbers).collect();

    let not_pick = game_numbers.len() - pick as usize;

    if pick as usize >= not_pick {
        let mut count = 0;
        while count < not_pick {
            game_numbers.remove(rand::thread_rng().gen_range(0..game_numbers.len()));
            count += 1;
        }
    } else {
        game_numbers.clear();
        while game_numbers.len() < pick as usize {
            let random = rand::thread_rng().gen_range(1..=numbers);
            if game_numbers.contains(&random) {
                continue;
            } else {
                game_numbers.push(random)
            }
        }
    }
    game_numbers.sort();
    game_numbers
}