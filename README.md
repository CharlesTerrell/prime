# prime
Program that checks a list of numbers for primes, and optionally does prime factorization.  Written in Rust for practice.

# prime --help output
    Check if numbers are prime

    Usage: prime [OPTIONS] <NUMBERS>...

    Arguments:
      <NUMBERS>...  One or more integers as inputs

    Options:
      -f, --factors     Find prime factors of each input
      -a, --all-primes  Find all prime numbers up to the largest input
      -h, --help        Print help
      -V, --version     Print version

# known bugs
Sometimes crashes with specific inputs.  Still looking for a pattern.
