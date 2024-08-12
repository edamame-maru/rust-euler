fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn largest_palindrome_product() -> u32 {
    let mut max_palindrome = 0;
    for i in (100..=999).rev() {
        for j in (100..=999).rev() {
            let product = i * j;
            if product <= max_palindrome {
                break;
            }
            if is_palindrome(product) {
                max_palindrome = product;
            }
        }
    }
    max_palindrome
}

fn main() {
    let result = largest_palindrome_product();
    println!("{}", result);
}
