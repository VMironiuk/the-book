use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to Guessing Game!");

    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read a line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number. Try again");
                break
            }
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }    
    }
}
