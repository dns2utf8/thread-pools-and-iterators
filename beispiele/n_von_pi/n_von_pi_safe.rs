use std::io::prelude::*;
use std::io;

fn main() {
    let pi = "3.1415926f32";

    let mut input = String::new();
    loop {
        print!("wie vielte Stelle? ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_bytes_read) => {
                //println!("{}", input);
                let stelle: usize = input.trim().parse().expect("ungültige Eingabe");
                println!("Gewünschte Stelle: '{:?}'", pi.chars().nth(stelle));
                input.clear();
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
