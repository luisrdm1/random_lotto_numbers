use std::collections::HashSet;
use clap::Parser;
use colored::*;
use lotto_quick_pick as lqp;
use lqp::Parameters;
use num_bigint::ToBigUint;

const HOW_MANY_GAMES: &str = "How many games would you like? (≥ 1; if 0, abort)";
const WHERE_START: &str = "Which number to start? (≥ 0)";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Shows the probability of winning a prize with the matched balls
    #[arg(short = 'm', long, value_name = "MATCHED-BALLS")]
    matched: Option<u8>,

    /// Stores the number of balls to hit a jackpot
    #[arg(short, long)]
    jackpot: Option<u8>,

    /// Sets the number of tickets that will be generated
    #[arg(short, long, value_name = "TICKETS", default_value_t = 1)]
    tickets: usize,

    /// Sets the starting number of the lottery game
    #[arg(short, long, value_name = "START-NUMBER")]
    start_number: u8,

    /// Sets the ending number of the lottery game
    #[arg(short, long, value_name = "END-NUMBER")]
    end_number: u8,

    /// Sets the quantity of numbers that will be picked for each ticket
    #[arg(short, long, value_name = "PICK")]
    pick: u8,
}

fn run(tickets: HashSet<Vec<u8>>) {
    let color_0 = "0".bright_green();
    for ticket in tickets {
        let _ = ticket
            .iter()
            .map(|num| {
                if *num < 10 {
                    print!("{color_0}{} ", num.to_string().bright_green());
                } else {
                    print!("{} ", num.to_string().bright_green());
                }
            })
            .collect::<Vec<_>>();
        println!();
    }
}

fn main() {
    let cli = Cli::parse();

    let parameters = Parameters::new(cli.tickets, cli.start_number, cli.end_number, cli.pick);

    let tickets = lqp::bundle(parameters);

    run(tickets);

    if let Some(balls) = cli.matched {
        let (chances, possibilities) = lqp::probability::odds(parameters, balls);
        if chances > 1.to_biguint().unwrap() {
            println!(
                "Your probability is approximately a {} over {} for that bundle.\n",
                &chances / &chances,
                &possibilities / &chances
            );
        } else {
            println!("Your probability is {chances} over {possibilities} for that bundle.\n",);
        }
    }
}

// Dead code for now
// fn run_interactive_mode() {
//     loop {
//         let mut count = 0;

//         let games = match lqp::input_into_number::<usize>(HOW_MANY_GAMES) {
//             0 => {
//                 eprintln!("{}", "Ok. Aborting.".red().bold());
//                 break;
//             }
//             x => x,
//         };

//         let start = lqp::input_into_number::<u8>(WHERE_START);

//         let where_end = format!("Say the last number, it should be greater than {start}.");

//         let end = lqp::input_into_number::<u8>(&where_end);

//         if end <= start {
//             eprintln!("{}", "We need at least two numbers. Aborting.".bold().red());
//             break;
//         }

//         let how_many_pick = format!(
//             "How many numbers to pick? Should be less than {}",
//             (end - start)
//         );

//         let pick = match lqp::input_into_number::<u8>(&how_many_pick) {
//             0 => {
//                 eprintln!(
//                     "{}",
//                     "Picking zero numbers won't help you. Aborting."
//                         .red()
//                         .bold()
//                 );
//                 break;
//             }
//             x => {
//                 if x > (end - start) {
//                     eprintln!(
//                         "{}",
//                         "You can't pick more numbers than you have!".bold().red()
//                     );
//                     break;
//                 } else {
//                     x
//                 }
//             }
//         };

//         let parameters = lqp::Parameters::new(games, start, end, pick);

//         while count < parameters.games {
//             let random_numbers = lqp::Parameters::generate_ticket(&parameters);

//             for num in random_numbers {
//                 let color_num = num.to_string().bright_green();
//                 if num < 10 {
//                     let zero = 0.to_string().bright_green();
//                     print!("{zero}{color_num} ");
//                 } else {
//                     print!("{color_num} ");
//                 }
//             }
//             println!();
//             count += 1;
//         }
//         eprintln!(
//             "This kind of game has {} possibilities.\n",
//             lqp::probability::combinations(end - start + 1, pick)
//         );
//     }
// }
