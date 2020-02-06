use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let secret = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();
    
    
    println!("Guess the number!");
    println!("ans: {}", secret);
    
    loop {
        println!(">");
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess: u32 = guess.trim().parse().expect("fatal: input was not a positive integer");

        println!("> {}", guess);
        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("Horray!");
                break;
            }
            Ordering::Greater => println!("too big"),
        }

    }

}
