// Problem 4

// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.

// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let mut break_again = false;
    let mut max = (0, 0, 0);
    for i in (100..1000).rev() {
        for j in (i..1000).rev() {
            let x = i as u32 * j as u32;
            if is_palindrome(x) && x > max.0  {
                max = (x, i, j);
            }
        }
    }
    println!("{} * {} = {}", max.1, max.2, max.0);
}

fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();

    let half = s.len() / 2;

    s.bytes().take(half).eq(s.bytes().rev().take(half))
}
