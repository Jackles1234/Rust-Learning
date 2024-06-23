extern crate rand; //Using rand crate as external dependency
use std::io;
use rand::Rng; //Rng trait defines the methods that rng's implement
use std::cmp::Ordering; // Enum

fn main() {
    println!("Guess the Number!");
    //rand::thread_rng gives us a particular rng that we are going to use. 
    //
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Input your guess!");
        //  :: indicates 'new' os an associated function of String type.    
        let mut guess = String::new();
        //*IMPORTANT: & indicates reference so you don't need to copy value to memory */
        // Read_line puts what the user types into the string we're passing it and returns a value ('io::Result')
        //The Result types are 'enumerations'. Enums aree a type that can have a fixed set of values that are called varients.
        //The Result types is used to encode error-handling info (expect). 
        io::stdin().read_line(&mut guess)
        //Error handling
        .expect("Failed to read line");
        // Second guess refers to first guess. trim eliminates whitespace. Parse on strings parses into some kind of number (u32 in this case)
        let guess: u32 = match guess.trim().parse() {//Shadowing the previous guess and converting it from one type to another. 
        Ok(num) => num,         //switching from expect to match allows for the program to handle the error rather than crash the program
        Err(_) => continue, // OK() will match the arms value and mathc expression will return the num and put inside the Ok value. Same with Err() with underscore being a catchall value
        };
        println!("Your guess: {}", guess);
        //Match expression is made up of arms. An arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arms pattern
        //Ex: guess = 50 and sn = 38. returns ordering::greater and comparies it to all of the arms. Eventually ordering::Greater matches with ordering::Greater
        match guess.cmp(&secret_number){ //cmp method compares tweo values and can be called on anything.
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  //Handling loop
            }
        }
    }
}