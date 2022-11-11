use random_lotto_numbers as rln;

fn main() {
    loop {
        let mut count = 0;

        let how_many_games = "How many games would you like? (≥ 1; if 0, abort)";
        let where_start = "Which number to start? (≥ 0)";

        let games = match rln::input_into_number::<u16>(how_many_games) {
            0 => {
                eprintln!("Ok. Aborting.");
                break;
            }
            x => x,
        };

        let start = rln::input_into_number::<u8>(where_start);

        let where_end = format!("Say the last number, it should be greater than {start}.");

        let end = rln::input_into_number::<u8>(&where_end);

        if end <= start {
            eprintln!("We need at least two numbers. Aborting.");
            break;
        }

        let how_many_pick = format!(
            "How many numbers to pick? Should be less than {}",
            (end - start)
        );

        let pick = match rln::input_into_number::<u8>(&how_many_pick) {
            0 => {
                eprintln!("Picking zero numbers won't help you. Aborting.");
                break;
            }
            x => {
                if x > (end - start) {
                    eprintln!("You can't pick more numbers than you have!");
                    break;
                } else {
                    x
                }
            }
        };

        let parameters = rln::Parameters::new(games, start, end, pick);

        while count < parameters.games {
            let random_numbers = rln::Parameters::generate_game(&parameters);

            for num in random_numbers {
                print!("{} ", num);
            }
            println!();
            count += 1;
        }
        println!(
            "This kind of game has {} possibilities.",
            rln::probability::combinations(end - start + 1, pick)
        );
    }
}
