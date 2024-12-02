fn main() {
    let mut x: i64 = 600851475143;
    let mut i: i64 = 2;
    while i < x {
        if x % i == 0 {
            x /= i;
            continue;
        }
        i += 1;
    }
    println!("{i}");
    return;
}
