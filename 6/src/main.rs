// Find the difference between the sum of the squares of the first 100 natural numbers
// and the square of the sum of the first 100 natural numbers.

// This is a nice problem to practice iterator magic on
fn sum_sq(n: u64) -> u64 {
    return (1..n+1).map(|x| x.pow(2)).sum();
}

fn sq_sum(n: u64) -> u64 {
    return (1..n+1).sum::<u64>().pow(2);
}

fn main() {
    let difference:u64 = sq_sum(100) - sum_sq(100);
    println!("Different: {}", difference);
}