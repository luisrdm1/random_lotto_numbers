use clap::Parser;
use colored::*;
use random_lotto_numbers as rln;
use rln::Parameters;

const HOW_MANY_GAMES: &str = "How many games would you like? (≥ 1; if 0, abort)";
const WHERE_START: &str = "Which number to start? (≥ 0)";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Shows the possibilities to the user
    #[arg(short = 'P', long, default_value_t = false)]
    possibilities: bool,

    /// Sets the number of games that will be generated
    #[arg(short, long, value_name = "GAMES", default_value_t = 1)]
    games: usize,

    /// Sets the starting number of the lottery
    #[arg(
        short,
        long,
        value_name = "START_NUMBER",
    )]
    start_number: u8,

    /// Sets the ending number of the lottery
    #[arg(
        short,
        long,
        value_name = "END-NUMBER",
    )]
    end_number: u8,

    /// Sets the quantity of numbers that will be picked by each number
    #[arg(
        short,
        long,
        value_name = "PICK",
    )]
    pick: u8,
}

fn main() {
    let cli = Cli::parse();

        let parameters = Parameters::new(cli.games, cli.start_number, cli.end_number, cli.pick);
        parameters.validate();

        rln::run(parameters);

    if let true = cli.possibilities {
        eprintln!(
            "This kind of game has {} possibilities.\n",
            rln::probability::combinations(cli.end_number - cli.start_number + 1, cli.pick)
        );
    }
    
}

fn run_interactive_mode() {
    loop {
        let mut count = 0;

        let games = match rln::input_into_number::<usize>(HOW_MANY_GAMES) {
            0 => {
                eprintln!("{}", "Ok. Aborting.".red().bold());
                break;
            }
            x => x,
        };

        let start = rln::input_into_number::<u8>(WHERE_START);

        let where_end = format!("Say the last number, it should be greater than {start}.");

        let end = rln::input_into_number::<u8>(&where_end);

        if end <= start {
            eprintln!("{}", "We need at least two numbers. Aborting.".bold().red());
            break;
        }

        let how_many_pick = format!(
            "How many numbers to pick? Should be less than {}",
            (end - start)
        );

        let pick = match rln::input_into_number::<u8>(&how_many_pick) {
            0 => {
                eprintln!(
                    "{}",
                    "Picking zero numbers won't help you. Aborting."
                        .red()
                        .bold()
                );
                break;
            }
            x => {
                if x > (end - start) {
                    eprintln!(
                        "{}",
                        "You can't pick more numbers than you have!".bold().red()
                    );
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
        eprintln!(
            "This kind of game has {} possibilities.\n",
            rln::probability::combinations(end - start + 1, pick)
        );
    }
}
