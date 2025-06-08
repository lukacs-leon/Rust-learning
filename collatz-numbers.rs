/*
This code is printing the collatz numbers from the START_NUM.
The collatz method halves the number if it's pairly, and 3x+1 if not pairly
*/
const START_NUM: i32 = 12;
fn main() {
    println!("this code is printing a collatz-number series from {}", START_NUM);
    let mut prev_num = START_NUM;
    let mut actual_num = 0;
    let mut downs = 0;
    let mut ups = 0;
    while actual_num != 1{
        if START_NUM % 2 == 1 {
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
}