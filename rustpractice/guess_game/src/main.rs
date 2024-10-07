fn main() {
    let secret_num = 23; // Hard-coded secret_num number
    let mut guess = 0; // Initial guess
    let mut num_guesses = 0; // Track the number of guesses

    // Define the check_guess function
    fn check_guess(guess: i32, secret_num: i32) -> i32 {
        if guess == secret_num {
            0
        } else if guess > secret_num {
            1
        } else {
            -1
        }
    }

    // Loop until the correct guess is made
    loop {
        num_guesses += 1; // Increment the number of guesses
        let result = check_guess(guess, secret_num);

        match result {
            0 => {
                println!("Congratulations! You guessed the correct number {} in {} attempts.", secret_num, num_guesses);
                break;
            }
            1 => println!("Your guess {} is too high!", guess),
            -1 => println!("Your guess {} is too low!", guess),
            _ => println!("Unexpected result: {}", result), // Catch-all pattern
        }

        // Simulate user input by incrementing the guess
        guess += 1;
    }
}