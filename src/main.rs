use clap::Parser;
use colored::Colorize;
use lotto_quick_pick::{self as lqp, Config, calculate_probability, generate_tickets};
use rand::rng;

/// Command-line lottery ticket generator.
///
/// Generates unique lottery tickets with configurable parameters
/// and displays winning probabilities.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Shows the probability of winning a prize with the matched balls
    #[arg(short = 'm', long, value_name = "MATCHED-BALLS")]
    matched: Option<usize>,

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
    pick: usize,
}

/// Display generated tickets with colored formatting.
fn display_tickets(tickets: &[lqp::Ticket]) {
    for ticket in tickets {
        for ball in ticket.balls() {
            print!("{} ", ball.to_string().bright_green());
        }
        println!();
    }
}

fn main() {
    let cli = Cli::parse();

    // Create configuration with error handling
    let config = match Config::new(cli.tickets, cli.start_number, cli.end_number, cli.pick) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", format!("Configuration error: {}", e).red().bold());
            std::process::exit(1);
        }
    };

    // Generate tickets
    let mut rng = rng();
    let tickets = match generate_tickets(&mut rng, &config) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", format!("Generation error: {}", e).red().bold());
            std::process::exit(1);
        }
    };

    // Display tickets
    display_tickets(&tickets);

    // Display probability if requested
    if let Some(matched_balls) = cli.matched {
        let total_balls = (cli.end_number - cli.start_number + 1) as usize;

        match calculate_probability(total_balls, cli.pick, matched_balls) {
            Ok((favorable, total)) => {
                // Simplify the fraction if possible
                let gcd = gcd_u128(favorable, total);
                let simplified_favorable = favorable / gcd;
                let simplified_total = total / gcd;

                if simplified_favorable == 1 {
                    println!(
                        "\nYour probability of matching {} balls is {} in {}",
                        matched_balls,
                        simplified_favorable.to_string().bright_yellow(),
                        simplified_total.to_string().bright_yellow()
                    );
                } else {
                    println!(
                        "\nYour probability of matching {} balls is approximately 1 in {}",
                        matched_balls,
                        (simplified_total / simplified_favorable)
                            .to_string()
                            .bright_yellow()
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    format!("Probability calculation error: {}", e).red().bold()
                );
            }
        }
    }
}

/// Calculate the greatest common divisor of two u128 numbers.
fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// TODO: Interactive mode - Implement interactive menu for user input
// This will allow users to configure lottery parameters through prompts
// instead of command-line arguments.
//
// Required features:
// - TODO: Interactive mode - Input loop with prompts for games, start, end, pick
// - TODO: Interactive mode - Validate user input with friendly error messages
// - TODO: Interactive mode - Allow retry on invalid input instead of exit
// - TODO: Interactive mode - Display menu with options (generate, calculate probability, exit)
// - TODO: Interactive mode - Show generated tickets in formatted output
// - TODO: Interactive mode - Calculate and display probabilities interactively
// - TODO: Interactive mode - Option to generate multiple batches in one session
//
// Implementation notes:
// - Use the existing Config::new() for validation (returns Result)
// - Use display_tickets() for output formatting
// - Use calculate_probability() for odds calculation
// - Keep I/O separate from library code (already refactored)
//
// Example interactive flow:
// ```
// fn run_interactive_mode() -> lqp::Result<()> {
//     loop {
//         println!("=== Lotto Quick Pick - Interactive Mode ===");
//         println!("1. Generate tickets");
//         println!("2. Calculate probability");
//         println!("3. Exit");
//
//         // Read user choice
//         // If generate: prompt for parameters, create Config, generate, display
//         // If calculate: prompt for parameters, call calculate_probability, display
//         // If exit: break
//     }
//     Ok(())
// }
// ```
