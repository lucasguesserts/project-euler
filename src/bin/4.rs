fn palindromic(n: i32) -> bool {
    let n = n.to_string();
    let n: Vec<char> = n.chars().collect();
    for i in 0..(n.len() / 2) {
        if n[i] != n[n.len() -1 - i] {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut m = 1;
    for i in 100..1000 {
        for j in i..1000 {
            let n = i * j;
            if palindromic(n) && m < n {
                m = n;
            }
        }
    }
    println!("{m}");
}
