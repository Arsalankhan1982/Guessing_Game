extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::process;


fn main() {
    loop{
        println!("Please Guess The Number!");
        let sc = rand::thread_rng().gen_range(1,101);
        let mut ar = 0;
        loop {
            println!("Please Input Your Guess.");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed To Read Line");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
                };
                println!("You have Guessed : {}", guess);
                match guess.cmp(&sc) {
                    Ordering::Less    => println!("Too Small !"),
                    Ordering::Greater => println!("Too Big !"),
                    Ordering::Equal   => {
                        println!("You Win !");
                        break;
                    }
                }    
                    ar=ar+1;
                if ar == 5 
                {
                    println!("Game Over");
                    process::exit(5)};
            }

                    println!("Do you Want To Continue = Y/N");
        let mut l = String::new();
        io::stdin().read_line(&mut l).expect("Failed to Read Line");
        if l!="y\n" {
            println!("Thanks For Playing");
            break;
            }
        }
    }