fn find(mut n: u64) -> u64 {
    let mut factor = 2;
    while n % factor == 0 {
        n /= factor;
    }
    factor = 3;
    while factor * factor <= n {
        while n % factor == 0 {
            n /= factor;
        }
        factor += 2;
    }
    if n > 2 {
        n
    } else {
        factor - 2
    }
}

fn main() {
    const NUMBER: u64 = 600851475143;
    let result = find(NUMBER);
    println!("{}", result);
}
