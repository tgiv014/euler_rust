// Find the sum of all natural numbers below 1000 that are divisible by 3 and/or 5

fn main() {
    let mut total = 0;
    
    // Per "The Book", this is the nice way to iterate
    for number in 1..1000 {
        if number % 3 == 0 || number % 5 == 0 {
            total += number;
        }
    }
    println!("Total: {}", total);
}