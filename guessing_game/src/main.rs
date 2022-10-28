use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please enter a number: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line !");
    println!("You Guessed {}",guess);
}
