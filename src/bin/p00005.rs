// Problem 5

// 2520 is the smallest number that can be divided by each
// of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible
// by all of the numbers from 1 to 20?

fn main() {
    const LIM: u64 = 20;

    let mut product: u64 = 1;

    for i in 2..LIM+1 {
        if is_prime(i) {
            let power = (LIM as f64).log(i as f64) as u64;
            println!("Prime: {}, Power: {}", i, power);
            product *= (i as f64).powi(power as i32) as u64;
        }
    }

    println!("The product is {}", product);
}

fn is_prime(x: u64) -> bool {
    let max = (x as f64).sqrt() as u64 + 1;

    for i in 2..max {
        if x % i == 0 {
            return false;
        }
    }
    true
}
