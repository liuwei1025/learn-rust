use rand::{ thread_rng, Rng };
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_num = thread_rng().gen_range(1, 101);
    // println!("The secret number is {}", secret_num);

    loop {
        println!("Please input you guess:");
        // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        // An associated function is implemented on a type, in this case String, rather than on a particular instance of a String.
        // Some languages call this a static method.
        let mut guess = String::new();
        io::stdin()
            // The & indicates that this argument is a reference,
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("The Error is {}", err);
                continue;
            }
        };
        println!("You guessed {}", guess);
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You wind!");
                break;
            }
        }
    }
}
