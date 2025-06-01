use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Enter a guess: ");
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
