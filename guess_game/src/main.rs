extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop_array();
    println!("Guess the number!");
    println!("Please input your guess:");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is {}", secret_number);
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>  {
                println!("Please input a number", );
                continue; 
            },
        };
            
        println!("You guessed: {}", guess);
        let is_bingo = check_guess(secret_number, guess);
        if is_bingo {
            break;
        }
        
    }
}

fn check_guess(secret: i32, guess: i32) -> bool{
    match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Too small!", );
                return false;
            },
            Ordering::Greater => {
                println!("Too Big!", );
                return false;
            },
            Ordering::Equal => {
                println!("Bingo!", );
                return true;
            },
        }
}

fn loop_array() {
    let arr = [1,2,3,4,5];
    for item in arr.iter() {
        println!("The value is {}", item)
    }
}