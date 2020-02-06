use rand::Rng;

fn main() {
    
    let secret = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();
    
    
    println!("Guess the number!");
    println!("ans: {}", secret);
    
    println!(">");
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("> {}", guess)

}
