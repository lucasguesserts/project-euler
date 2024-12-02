fn palindrome(n: i32) -> bool {
    return (n % 10 != 0) && (n == reverse(n));
}

fn reverse(mut n: i32) -> i32 {
    let mut r = 0;
    while n > 0 {
        r *= 10;
        r += n % 10;
        n /= 10;
    }
    return r;
}

fn main() {
    let mut m = 1;
    for i in 100..1000 {
        for j in i..1000 {
            let n = i * j;
            if palindrome(n) && m < n {
                m = n;
            }
        }
    }
    println!("{m}");
}
