use clap::Parser;
use primal::Sieve;
use integer_sqrt::IntegerSquareRoot;
use std::fmt::Write;

#[derive(Parser, Debug)]
/// Check if numbers are prime
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Find prime factors of each input
    #[arg(short, long)]
    factors: bool,

    /// Find all prime numbers up to the largest input
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
    println!("Largest input: {}", sorted_input[input_bound]);
    if cli.all_primes {
        println!("sieving all primes up to {}", sorted_input[input_bound]);
        prime_sieve = Sieve::new(sorted_input[input_bound]);
        for one_prime in prime_sieve.primes_from(2) {
            println!("{}", one_prime);
        }
    } else {
        println!("sieving primes up to 1+sqrt({})", sorted_input[input_bound]); 
        prime_sieve = Sieve::new(sorted_input[input_bound].integer_sqrt()+1);
        for one_prime in prime_sieve.primes_from(2) {
            println!("{}", one_prime);
        }
    }

    println!("about to loop through inputs");
    for item in sorted_input {
        println!("looping through inputs...");
        if prime_sieve.is_prime(item) {
            println!("{} is prime", item);
        } else {
            println!("{} is NOT prime", item);
        }
        println!("Now we check for factors");
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
