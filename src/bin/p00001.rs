fn main() {
    let mut sum = 0;
    for i in 0..1000 {
        if div_by_3(i) || div_by_5(i) {
            sum = sum + i;
        }
    }
    println!("The sum is {}", sum);
}

fn div_by_3(x: i32) -> bool {
    x % 3 == 0
}

fn div_by_5(x: i32) -> bool {
    x % 5 == 0
}
