use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let num = rand::rng().random_range(1..=10);

    loop {
        println!("Guess a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read");

        let guess_num: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Guess: {guess_num}");

        match guess_num.cmp(&num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater=> println!("too big"),
            Ordering::Equal=> {
                println!("You win");
                break;
            }
        }
    }
}
