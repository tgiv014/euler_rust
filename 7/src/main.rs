// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10001st prime number?

// This is not terribly efficient, but it is a nice idiomatic way to describe a prime number
fn is_prime(n: u64) -> bool {
    // 1 is not considered prime
    if n == 1 {
        return false;
    }

    return !(2..n).any(|x| n % x == 0);
}

fn main() {
    let mut num_primes: u64 = 0;

    for i in 1.. {
        if is_prime(i) {
            num_primes += 1;
        }
        if num_primes == 10001 {
            println!("{}th prime: {}", num_primes, i);
            return;
        }
    }
}