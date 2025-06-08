/*
This code is printing the collatz numbers from the START_NUM.
The collatz method halves the number if it's pairly, and 3x+1 if not pairly
*/
const START_NUM = 12;
println!("this code is printing a collatz-number series from {}", START_NUM);
fn main() {
    let mut prev_num = START_NUM;
    let mut actual_num = 0;
    let mut downs = 0;
    let mut ups = 0;
    if START_NUM % 1 == 1 {
        actual_num = prev_num * 3 +1;
        ups += 1;
        println!("{} \u{2191}", actual_num);
    } else {
        actual_num = prev_num / 2;
        downs += 1;
        println!("{} \u{2193}", actual_num);
    }
    prev_num = actual_num;
}