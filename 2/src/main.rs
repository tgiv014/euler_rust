// Find the sum of all even terms in the Fibonacci sequence below 4 million
fn main() {
    let mut total = 0;
    let mut previous = 1;
    let mut current = 1;

    // "current" is the accumulated number
    while current < 4000000 {
        // Accumulate!
        if current % 2 == 0 {
            total += current;
        }

        // Calculate the new piece of state before modifying state
        let next = previous + current;
        
        // Modify State
        previous = current;
        current = next;
    }
    println!("Total: {}", total);
}