fn max(pulse_vec: Vec<i32>) -> i32 {
    let mut max_pulse: i32 = 0;
    for i in pulse_vec {
        if i > max_pulse {
            max_pulse = i;
        }
    }
    max_pulse
}
fn main () {
    println! ("This application is make some statistics with your pulse.");
    let pulse_vec = vec![54, 65, 86, 45, 65, 96, 120, 57, 192, 364, 76, 87];
    let max_pulse = max(pulse_vec);
    println!("{}", max_pulse);
}

