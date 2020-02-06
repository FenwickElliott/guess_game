
fn main() {
    println!("Guess the number!");

    let mut guess = String::new();

    println!(">");

    std::io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("> {}", guess)

}
