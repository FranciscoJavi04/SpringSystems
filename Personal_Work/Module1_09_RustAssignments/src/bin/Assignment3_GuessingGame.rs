fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret: i32 = 37;

    let mut guesses_taken = 0;
    let mut guess = 0;

    loop {
        guesses_taken = guesses_taken + 1;

        if guesses_taken == 1 {
            guess = 30;
        } else if guesses_taken == 2 {
            guess = 50;
        } else if guesses_taken == 3 {
            guess = 40;
        } else {
            guess = 37;
        }

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {guess}: correct!");
            break;
        } else if result == 1 {
            println!("Guess {guess}: too high!");
        } else {
            println!("Guess {guess}: too low!");
        }
    }

    println!("It took {guesses_taken} guesses.");
}
