# prime
Program that checks a list of numbers for primes, and optionally does prime factorization.  Written in Rust for practice.

# output of 'prime --help'
    Check if numbers are prime

    Usage: prime [OPTIONS] <NUMBERS>...

    Arguments:
      <NUMBERS>...  One or more integers as inputs

    Options:
      -f, --factors     Find prime factors of each input
      -a, --all-primes  Find all prime numbers up to the largest input
      -h, --help        Print help
      -V, --version     Print version

# output of 'prime -f 42 1024 17'
    Largest input: 1024
    sieving primes up to 1+sqrt(1024)
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
    about to loop through inputs
    looping through inputs...
    17 is prime
    Now we check for factors
    17: 17^1 
    looping through inputs...
    42 is NOT prime
    Now we check for factors
    42: 2^1 3^1 7^1 
    looping through inputs...
    1024 is NOT prime
    Now we check for factors
    1024: 2^10

# known bugs
Sometimes crashes with specific inputs.  Still looking for a pattern.  Thus why the output is currently so verbose.
