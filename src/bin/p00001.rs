// Problem 1

// If we list all the natural numbers below 10 that are multiples of 3 or 5,
// we get 3, 5, 6 and 9. The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
    const MAX_VAL: u32 = 1_000;

    let mut sum = 0;

    for i in 0..MAX_VAL {
        if div_by_3(i) || div_by_5(i) {
            sum = sum + i;
        }
    }
    println!("The sum is {}", sum);
}

fn div_by_3(x: u32) -> bool {
    x % 3 == 0
}

fn div_by_5(x: u32) -> bool {
    x % 5 == 0
}
