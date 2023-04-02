extern crate rand;
use rand::{thread_rng, Rng};

fn check_a_guess(guess:u16, target:u16, guesses_left:u16) -> bool {
    
    if guess == target{
        println!("Congratulations, your guess was correct!");
        return true;
    } else if guess < target {
        println!("Your guess was too low!");
        println!("You have {} guesses left", guesses_left);
        return false;
    } else if guess > target {
        println!("Your guess was too high!");
        println!("You have {} guesses left", guesses_left);
        return false;
    } else {
        println!("Error: illegal state");
        return false;
    }
}

fn get_a_guess() -> u16 {
    let mut guess_string = String::new();
    
    print!("Guess a number between 0 and 500!\n> ");
    std::io::stdin().read_line(&mut guess_string).expect("Failed to read user input");
    let guess:u16 = guess_string.trim().parse().unwrap();
    return guess;
}

fn main() {
    let mut rng = thread_rng();
    let num_to_guess:u16 = rng.gen::<u16>() % 500;
    
    let mut guess:u16;
    let mut number_guessed:bool = false;
    let mut guesses_left: u16 = 5;
        
    //println!("Number to Guess: {}", num_to_guess);
    while number_guessed != true && guesses_left > 0 {
        guess = get_a_guess();
        println!("You guessed {}!",guess);
        guesses_left = guesses_left - 1;
        number_guessed = check_a_guess(guess, num_to_guess, guesses_left);
    }
    
    println!("The corrent answer was {}", num_to_guess);
    println!("Thanks for playing!");
    return;
}
