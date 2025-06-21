use std::{cmp::Ordering, io};
use rand::Rng;
use colored::*;

fn main() {
    println!("{}","***Guess the number game***".yellow());

    loop {
        let mut num: String= String::new();
        
        let ran: i32=rand::thread_rng().gen_range(1, 101);
        println!("Random number is {}",ran);
        
        println!("Enter a number");
        io::stdin().read_line(&mut num).expect("read line failed");
        println!("Input number {}", num);

        let num: i32 =match num.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match num.cmp(&ran) {
            Ordering::Greater => println!("{}","Entered number is greater".red()),
            Ordering::Less => println!("{}","Entered number is lesser".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            },
        }
    }
}
