mod commands;

use clap::{Parser, Subcommand};

// Structs to setup and hold argument information
#[derive(Parser)]
#[command(version = "0.1.0", about, long_about = None, author = "Lyra" )]
struct Args {
    #[command(subcommand)]
    command: Commands,
    /// Input file; can be binary or text for encoding, or an image for decoding.
    #[clap(short, long)]
    input: String,
    /// Output file; must be an image for encoding, or binary or text for decoding.
    #[clap(short, long)]
    output: String,
    /// Enable verbose output
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Encode,
    Decode,
}

fn main() {
    // Print the banner
    println!(
        "  █████▒█     █░ ██▓▓█████
▓██   ▒▓█░ █ ░█░▓██▒▓█   ▀
▒████ ░▒█░ █ ░█ ▒██▒▒███
░▓█▒  ░░█░ █ ░█ ░██░▒▓█  ▄
░▒█░   ░░██▒██▓ ░██░░▒████▒
▒ ░   ░ ▓░▒ ▒  ░▓  ░░ ▒░ ░
░       ▒ ░ ░   ▒ ░ ░ ░  ░
░ ░     ░   ░   ▒ ░   ░
            ░     ░     ░  ░
                                 "
    );
    println!("Firewire's Image Encoding.");

    // Parse out the arguments
    let args = Args::parse();

    // Print configuration from args if we are in verbose mode
    if args.verbose {
        println!("[CONFIG] Command: {:?}", args.command);
        println!("[CONFIG] Input: {:?}", args.input);
        println!("[CONFIG] Output: {:?}", args.output);
    }

    // Match and run the supplied subcommand
    // If a function returns an error result, print out the error message and exit
    match args.command {
        Commands::Encode => {
            if let Err(e) = commands::encode::run(args.input, args.output) {
                eprintln!("[ERROR] {}", e);
            };
        }
        Commands::Decode => {
            if let Err(e) = commands::decode::run(args.input, args.output) {
                eprintln!("[ERROR] {}", e);
            }
        }
    }
}
