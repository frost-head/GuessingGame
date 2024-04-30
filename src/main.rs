use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colorz::{Colorize, xterm};




fn main() {
    println!("Welcome to the Guessing game!");
    println!("Please enter your guess :");
    let num = rand::thread_rng().gen_range(0..=100);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Gailes to read");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&num) {
            Ordering::Equal => {println!("{}","You win!".green()); break;},
            Ordering::Greater => println!("{}","Too big".red()),
            Ordering::Less => println!("{}","Too small".red()),
        }
    
    }    
}
