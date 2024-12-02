use std::cmp;
use std::collections::HashMap;

fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut factor: HashMap<i32, u32> = HashMap::new();
    for p in primes {
        factor.insert(p, 0);
    }
    for i in 1..=20 {
        for p in primes {
            let mut n = i;
            let mut count = 0;
            while n > 1 {
                if n % p == 0 {
                    count += 1;
                    n /= p;
                    continue;
                }
                break;
            }
            factor.insert(p, cmp::max(factor.get(&p).copied().unwrap_or(0), count));
        }
    }
    let mut m = 1;
    for (p, f) in factor {
        m *= p.pow(f);
    }
    println!("{m}");
}

//// slower but working solution:
// fn main() {
//     let mut x = 21;
//     'outer: loop {
//         for i in 1..=20 {
//             if x % i != 0 {
//                 x += 1;
//                 continue 'outer;
//             }
//         }
//         break;
//     }
//     println!("{x}");
// }

