fn max_pulse(pulse_vec: Vec<i32>) -> i32 {
    let mut max_pulse: i32 = 0;
    for i in pulse_vec {
        if i > max_pulse {
            max_pulse = i;
        }
    }
    max_pulse
}
fn pulse_avarage(pulse_vec: Vec<i32>) -> f64 {
    let mut total_pulse_sum = 0;
    for i in pulse_vec {
        total_pulse_sum += i;
    }
    let pulse_avarage = total_pulse_sum / pulse_vec.len()
    pulse_avarage
}
fn main () {
    println! ("This application is make some statistics with your pulse.");
    let pulse_vec = vec![54, 65, 86, 45, 65, 96, 120, 57, 192, 364, 76, 87];
    let max_pulse = max_pulse(pulse_vec);
    let pulse_avarage = pulse_avarage(pulse_vec);
    println!("Based on your data, the highest heart rate was {} bpm", max_pulse);
    println!("Based on your data, the avarage pf your heart rate was {} bpm", pulse_avarage);
}

