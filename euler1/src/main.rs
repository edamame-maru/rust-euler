fn sum_of_multiples(limit: u32) -> u32 {
    let mut sum = 0;
    for n in 1..limit {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum
}

fn main() {
    let limit = 1000;
    let result = sum_of_multiples(limit);
    println!("{}", result);
}
