use std::collections::HashMap;

fn main () {
    let mut n: i64 = 1;
    loop {
        println!("n = {n}");
        let t = (n + 1) * n / 2;
        println!("t = {t}");
        let mut factor: HashMap<i64, i64> = HashMap::new();
        let mut x = t;
        let mut i = 2;
        while x > 1 && i <= x {
            let mut count = 0;
            while x % i == 0 {
                count += 1;
                x /= i;
            }
            if count > 0 {
                factor.insert(i, count);
            }
            i += 1;
        }
        let mut ndiv = 1;
        print!("factors = [");
        for (k, v) in &factor {
            print!("{}: {}, ", k, v);
        }
        println!("]");
        for (_, v) in &factor {
            ndiv *= v + 1;
        }
        println!("ndiv = {ndiv}");
        if ndiv > 500 {
            println!("\nAnswer of the problem: {t}");
            break;
        }
        n += 1;
        println!("");
    }
    return;
}
