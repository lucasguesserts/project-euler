fn main () {
    let mut primes: Vec<i64> = Vec::new();
    primes.push(2);
    primes.push(3);
    let mut x = 3;
    'outer: while primes.len() < 10_001 {
        x += 2;
        for &p in &primes {
            if x % p == 0 {
                continue 'outer;
            }
        }
        primes.push(x);
    }
    println!("{}", primes.last().unwrap_or(&0));
}