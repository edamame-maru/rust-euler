fn main() {
    let limit: i32 = 4000000;
    let mut sum: i32 = 0;

    let mut a: i32 = 1;
    let mut b: i32 = 1;

    while b < limit {
        if b % 2 == 0 {
            sum += b;
        }
    }

    let c: i32 = a + b;
    a = b;
    b = c;

    println!("{}", sum);
}