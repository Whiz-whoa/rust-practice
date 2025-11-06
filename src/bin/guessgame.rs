use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game Here");
    println!("what number between 0 and 100");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("guess panraa olungaa!!");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid number");
                continue;
            }
        };

        println!("guess {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("shmol"),
            Ordering::Greater => println!("beeg"),
            Ordering::Equal => {
                println!("Good Job");
                break;
            }
        }
    }
}
