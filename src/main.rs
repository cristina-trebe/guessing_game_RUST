use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    //generate a random number to guess
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}"); // just for testing the random secret_number

    loop {
        println!("Please input your guess:");
    
        let mut guess= String::new();
    
        //getting input from the keyboard 
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        //convert String as input into a real number
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        //and then printing the input
        println!("You guessed: {guess}");
    
    
        //Comparing "guess" to "secret_number"
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                                println!("You win!");
                                break;
                               }
        }

    }

    
}
