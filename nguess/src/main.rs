extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let guess_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please enter a number");
        
        let mut user_guess = String::new();
        
        io::stdin().read_line(&mut user_guess).expect("Failed to read user input!");

        let user_guess: u32 = match user_guess.trim().parse() {
         Ok(num) => num,
         Err(_) => {
            println!("You must enter a number");
            continue;
         }  
        };

        match user_guess.cmp(&guess_num) {
            Ordering::Equal => {
                println!("Yo, you're correct");
                break;
            },
            Ordering::Greater => println!("Yo, you entered higher"),
            Ordering::Less => println!("Yo, you entered lower")
        }

    } 
}
