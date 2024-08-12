fn sum_even_fibonacci(limit: u32) -> u32 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 2;

    while b <= limit {
        if b % 2 == 0 {
            sum += b;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    sum
}

fn main() {
    let limit = 4_000_000;
    let result = sum_even_fibonacci(limit);
    println!("{}", result);
}
