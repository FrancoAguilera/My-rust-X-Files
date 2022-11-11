use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(0..101);

    println!("\n=================The Guessing Game=================\n");
    println!("Enter your number guess (0..100):");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line!");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win :)".green());
                println!("The secret number was: {}", secret_number);
                break;
            }
        }
    }
}
