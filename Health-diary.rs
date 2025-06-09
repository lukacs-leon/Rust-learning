use std::io;
fn main () {
    println! ("This application is make some statistics with you r pulse.");
    let mut pulse_vec = vec! [];
    let mut add_more_pulse = true;
    println!("Pleas enter your first pulse what you want to use.");
    let mut pulse_input = String ::new();
    println!("Pleas enter you r first pulse what you want to use .");
    io::stdin()
    .read_line(&mut pulse_input )
    .expect("ERROR! Something went wrong! We couldn't read the data.");
    let pulse : u8 = pulse_input;
        .trim()
        .parse()
        .expect("ERROR! You didn't include a number!");
    pulse_vec.push(pulse);
    while add_more_pulse {
        println!("Pleas enter your next pulse.")
        let mut pulse_input = String::new();
        io::stdin()
            .read_line(&mut pulse_input )
            .expect("ERROR! Something went wrong! We couldn't read the data.");
        let pulse : u8 = pulse_input
            .trim()
            .parse()
            .expect("ERROR! You didn't include a number!");
        pulse_vec.push(pulse);
        let mut add_more_pulse_input = String::new();
        println!("Do you want to add more pulse? (Y/N)");
        io::stdin()
            .read_line(&mut name)
            .expect("ERROR! Something went wrong! We couldn't read the data.");
        let name = name.trim();
        if add_more_pulse_input == "N" {
            add_more_pulse = false;
        } else if add_more_pulse_input == "Y" {
            add_more_pulse_input = true;
        } else {
            println!("Somthing went wrong!")
        }
      }
}

