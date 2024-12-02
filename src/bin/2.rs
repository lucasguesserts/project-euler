fn main() {
    let mut f1 = 0;
    let mut f2 = 1;
    let mut f = f1 + f2;
    let mut sum = 0;
    while f < 4_000_000 {
        sum += if f % 2 == 0 { f } else { 0 };
        f1 = f2;
        f2 = f;
        f = f1 + f2;
    }
    println!("sum = {sum}");
    return;
}
