use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    
    let guess_splitted: String = guess.split_whitespace().collect();
    
    let guess_int: i32 = guess_splitted.parse().unwrap();

    let your_guess_plus_five: i32 = plus_five(guess_int);
    
    println!("Now you guessed plus five: {your_guess_plus_five}");

}

fn plus_five(x: i32) -> i32{
    return x+5
}
