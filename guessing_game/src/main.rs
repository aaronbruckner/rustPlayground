use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number!");

    loop {
        println!("Please enter your number...");

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
