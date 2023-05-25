# prime
Program that checks a list of numbers for primes, and optionally does prime factorization.  Written in Rust for practice.

# output of 'prime --help'
    Check if numbers are prime

    Usage: prime [OPTIONS] <NUMBERS>...

    Arguments:
      <NUMBERS>...  One or more integers as inputs

    Options:
      -f, --factors     Find prime factors of each input
      -a, --all-primes  Print all prime numbers up to the largest input
      -h, --help        Print help
      -V, --version     Print version

# output of 'prime -f 42 1024 17 6 43 365'
    6 is NOT prime
    6: 2^1 3^1 
    17 is prime
    17: 17^1 
    42 is NOT prime
    42: 2^1 3^1 7^1 
    43 is prime
    43: 43^1 
    365 is NOT prime
    365: 5^1 73^1 
    1024 is NOT prime
    1024: 2^10

# output of 'prime -a 37'
    2
    3
    5
    7
    11
    13
    17
    19
    23
    29
    31
    37
    37 is prime

# known bugs
Still needs unit tests.
