use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let _secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret is {}", _secret_number);

    loop {
        println!("Please input your guess");


        // Create a mutable variable named guess which holds a new instance of String
        // The "::" indicates that new is an associated function of the String type
        let mut guess = String::new();

        // stdin returns an instance of std::io::Stdin which handles user input
        // We then read the user input into a reference of the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too small!"),
        }
    }
}
