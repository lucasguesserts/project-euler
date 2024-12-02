fn main () {
    let mut primes: Vec<i64> = Vec::new();
    primes.push(2);
    primes.push(3);
    let mut x: i64 = 3;
    'outer: while x < 2_000_000 {
        x += 2;
        for p in &primes {
            if x % p == 0 {
                continue 'outer;
            }
        }
        primes.push(x);
    }
    let mut sum: i64 = 0;
    for p in &primes {
        sum += p;
    }
    println!("{sum}");
    return;
}