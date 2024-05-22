fn main() {
    let limit = 4000000;
    let mut sum = 0;

    let a = 1;
    let b = 1;

    while b < limit {
        if b % 2 == 0 {
            sum += b;
        }
    }

    let mut c = a + b;
    a = b;
    b = c;

    println!("{sum}");
}