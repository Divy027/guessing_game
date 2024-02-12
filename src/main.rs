
use std::{cmp::Ordering, io, u32};
use rand::Rng;
use colored::{self, Colorize};
fn main() {
    println!("Welcome to guessing game ");
    
    
    let secret_number = rand::thread_rng().gen_range( 1, 101); //random number generation
    println!("the secret number is {}",secret_number);

    loop {
        println!("Please input your number :");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess :u32 = match guess.trim().parse(){ //parse returns OK (sucess) or Err(expect) so match can be used to handle this better
            Ok(num) => num, // if sucess then return num
            Err(_) => continue, // `_` = catches all error 
            // if err then continue 
        };
        // trim = remove whitespace
        //parse = will convert string to type annoted 

        println!("you guessed {}",guess);
        match guess.cmp(&secret_number){ //If /else , cmp  returns Ordering(-1,0,1)
            Ordering::Less => println!("{}","Too Small!".red()),//-1
            Ordering::Greater => println!("{}","Too Big!".red()),//1
            Ordering::Equal => {//0
                println!("{}","You Win!".green());
                break;
            },
            
        }
    }

    

}
