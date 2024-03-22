use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please Enter you're guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed : {guess}");
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too Small!"),
            std::cmp::Ordering::Equal => {
                println!("Yo Got it Right Nigga!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too Big!,That's what she said"),
        }
    }
}
