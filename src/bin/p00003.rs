// Problem 3

// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

fn main() {
    const VAL: u64 = 600851475143;

    let mut n = VAL;
    let mut last_factor: u64 = 1;

   divide_out(&mut n, 2, &mut last_factor);

    let mut i = 3;
    while i <= n {
        divide_out(&mut n, i, &mut last_factor);
        i += 2;
    }

    println!("The largest prime of {} is {}", VAL, last_factor);
}

fn divide_out(x: &mut u64, n: u64, factor: &mut u64) {
    if *x % n == 0 {
        *x /= n;
        while *x % n == 0 {
            *x /= n;
        }
        *factor = n;
    }
}
