// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindrome(n: u64) -> bool {
    // Use string processing to make this easy
    // We only need to check the length of the string rounded down
    let checkstring = n.to_string();
    let numtocheck = checkstring.len() / 2;
    
    // Because we know the string is made of 0-9, we can use bytes
    // Select half of our string and compare it to the other half
    // The iterator eq() function is convenient for this
    checkstring.bytes().take(numtocheck).eq(checkstring.bytes().rev().take(numtocheck))
}

fn main() {
    let mut largest: u64 = 0;
    for i in (1..1000).rev() {
        for j in (1..1000).rev() {
            let number = i*j;
            if is_palindrome(number) && number > largest {
                println!("New largest palindrome: {}", number);
                largest = number;
            }
        }
    }
}