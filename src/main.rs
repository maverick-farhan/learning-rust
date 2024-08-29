// File name: src/main.rs
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("\t -----GUESS THE NUMBER!-----\t");
    let secret_number = rand::thread_rng().gen_range(1..=100);
loop{
    println!("Type 'Ctrl+C' to exit the game");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess:  u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You guessed: {}",guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("TOO SMALL"),
        Ordering::Greater => println!("TOO GREATER"),
        Ordering::Equal => {
            println!("YOU WIN!");
            break;
        },
    }
}
}
