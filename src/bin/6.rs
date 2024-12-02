fn main () {
    let mut sum_of_squares: i64 = 0;
    let mut square_of_sum: i64 = 0;
    for i in 1..=100 {
        sum_of_squares += i * i;
        square_of_sum += i;
    }
    square_of_sum = square_of_sum * square_of_sum;
    let diff = square_of_sum - sum_of_squares;
    println!("{diff}");
}