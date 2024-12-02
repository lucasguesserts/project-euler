fn main () {
    for a in 1..1000 {
        for b in a..1000 {
            let fc = f64::sqrt((a * a + b * b) as f64);
            let c = fc.round() as i32;
            if f64::abs(fc - (c as f64)) < 1.0e-8 {
                if a + b + c == 1000 {
                    println!("{}", a * b * c);
                    return;
                }
            }
        }
    }
    println!("not found");
    return;
}