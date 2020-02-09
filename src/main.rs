use std::cmp::Ordering;
use std::io::{self, Write};
use rand::Rng;

fn main() {
    
    let ans = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number! (You have 10 guesses)");
    println!("ans: {}", ans);
    
    for _ in 0..10 {
        let mut guess = String::new();
        
        print!("> ");
        io::stdout().flush().unwrap();
        
        io::stdin().read_line(&mut guess)
        .ok()
        .expect("fatal: failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input was not a positive integer, please try again");
                continue
            }
        };

        match guess.cmp(&ans) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("Horray! {} was correct", guess);
                return;
            }
            Ordering::Greater => println!("too big"),
        }
    }

    println!("I'm sorry, you're out of guesses. The answer was: {}", ans);
}
