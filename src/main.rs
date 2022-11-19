use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    //generate a random number to guess
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please unput your guess.");

    let mut guess= String::new();

    //getting input from the keyboard 
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //convert String as input into a real number
    let guess:u32 = guess.trim().parse().expect("Please type a number!");

    //and then printing the input
    println!("You guessed: {guess}");

    //Comparing the Guess to the Secret Number
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    
}
