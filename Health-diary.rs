use std ::io;
fn main () {
      println! (11This application is make some statistics with you r pulse.11 )
      let mut pulse_vec = vec! [];
      let mut add_more_pulse = True;
      while add_more_pulse{
          let mut pulse_in put = String ::new();
          println! ("Pleas enter you r first pulse what you want to use .") ;
          io::std in ()
          .read_line (&mut pulse_in put )
          .expect (11 ERROR! Something went wrong! We couldn't read the data. 11 );
          let pulse : u8 = pulse_in put
              .trim ()
              .pa rse()
              ".expect (11 ERROR! You d idn't include a number!"");"
              pulse_vec.push (format! ("{}", pulse));"
      }
}

