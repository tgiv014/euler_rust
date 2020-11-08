// Problem:
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

// 5 * 7 * 13 * 29 = 13195
fn is_prime(n: u64) -> bool {
    // 1 is not considered prime
    if n == 1 {
        return false;
    }

    // Iterate to determine if this number is prime
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut current: u64 = 600851475143;
    let mut test = 1;
    while test < current {
        // Skip all non-prime numbers
        if !is_prime(test) {
            test += 1;
            continue;
        }
        if current % test == 0 {
            current = current / test;
            println!("Found prime factor: {}", test);
        }
        test += 1;
    }
    println!("Final prime factor: {}", current);
}