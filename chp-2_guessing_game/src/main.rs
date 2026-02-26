use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello to guessign game!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // println!("{secret_number}");
        println!("The given guess is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }
}
