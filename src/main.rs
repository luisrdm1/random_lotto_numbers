use rand::Rng;
use std::io;
use std::str::FromStr;

fn main() {
    let mut count = 0;

    let games = input_into_number::<u16>("How many games would you like?".to_owned());

    let numbers = input_into_number::<u8>(String::from("How many numbers to choose from?"));

    let pick = input_into_number::<u8>(String::from("How many numbers to pick from?"));

    while count < games {
        let random_numbers = generate_numbers(numbers, pick);

        for num in random_numbers {
            print!("{} ", num);
        }
        println!();
        count += 1;
    }
}

fn input_to_u8(string: String) -> u8 {
    loop {
        println!("{string}");
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line.");
        match value.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn input_into_number<T: FromStr>(string: String) -> T {
    loop {
        println!("{string}");
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line.");
        match value.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn generate_numbers(numbers: u8, pick: u8) -> Vec<u8> {
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
