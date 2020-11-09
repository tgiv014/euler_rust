// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn is_fully_divisible(n: u64, k: u64) -> bool {
    // We already know that 1 is fine, start from 2
    // We can use fun iterator magic to assert that n is divisble by all k from 1 to 20
    return (2..k+1).all(|k| n % k == 0);
}

fn main() {
    let mut number: u64 = 1;
    
    println!("This may take a small while...");

    while !is_fully_divisible(number, 20) {
        number += 1;
    }

    println!("Smallest qualifying number:  {}", number);
}