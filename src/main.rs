use clap::Parser;
use primal::Sieve;
use std::fmt::Write;

#[derive(Parser, Debug)]
/// Check if numbers are prime
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Find prime factors of each input
    #[arg(short, long)]
    factors: bool,

    /// Print all prime numbers up to the largest input
    #[arg(short, long)]
    all_primes: bool,

    /// One or more integers as inputs
    #[arg(required = true)]
    numbers: Vec<usize>,
}

fn main() {
    let cli = Cli::parse();
    let mut sorted_input: Vec<usize> = cli.numbers;
    let prime_sieve: Sieve;

    sorted_input.sort_unstable();
    sorted_input.dedup();
    let input_bound: usize = sorted_input.len()-1;
    prime_sieve = Sieve::new(sorted_input[input_bound]);
    if cli.all_primes {
        for one_prime in prime_sieve.primes_from(2) {
            println!("{}", one_prime);
        }
    }

    for item in sorted_input {
        if prime_sieve.is_prime(item) {
            println!("{} is prime", item);
        } else {
            println!("{} is NOT prime", item);
        }
        if cli.factors {
            let mut full_factor: String = format!("{}: ", item); 
            let factors = prime_sieve.factor(item).unwrap();
            for fact_parts in factors {
                write!(full_factor, "{}^{} ", fact_parts.0, fact_parts.1).unwrap();
            }
            println!("{}", full_factor);
        }
    }
}
