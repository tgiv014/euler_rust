// Find the sum of all primes below two million
// This is largely an optimization problem. We need to find a better way to check if a number is prime
// For starters, we actually only need to check numbers up to sqrt(n)

fn is_prime(n: u128) -> bool {
    // 2 is prime!
    if n <= 3 {
        return n > 1;
    }
    if (n % 2 == 0) || (n % 3 == 0) {
        return false;
    }
    let mut i:u128 = 5;
    while i.pow(2) <= n {
        if (n % i == 0) || (n % (i + 2) == 0) {
            return false;
        }
        i += 6;
    }
    return true;
}

fn main() {
    let mut sum:u128 = 0;
    for n in 1..2000000 {
        if is_prime(n) {
            sum += n;
        }
    }
    println!("Sum: {}", sum);
}