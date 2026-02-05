use rand::Rng;
use std::io;
use std::process;

fn is_single_code_point(s: &str) -> bool {
    s.chars().count() == 1
}

fn main() {
    let mut guess_count_left: i8 = 5;

    // OWNERSHIP: `words` owns the Vector. The strings inside are static references `&'static str`.
    let words: Vec<&str> = vec!["apple", "library", "potato", "abra-kadabra", "volcano"];
    let secret_number = rand::rng().random_range(1..=words.len() - 1);

    // OWNERSHIP: `selected_word` is a reference (`&str`).
    // It borrows the string data from inside the `words` vector.
    // Ownership does NOT change here; `words` still owns the data.
    let selected_word: &str = &words[secret_number];

    let total_chars:usize = selected_word.chars().count();

    // OWNERSHIP: `guessed_chars` is created here and owned by `main`.
    // It is mutable because we will modify it (push to it) later.
    let mut guessed_chars: Vec<String> = Vec::new();
    
    

    //println!("Hello, world! {selected_word}");

    loop {
        println!("Please guess a character.");

        // OWNERSHIP: `guess` is created here (owned by the loop iteration).
        // It is a new String every time the loop runs.
        let mut guess = String::new();

        io::stdin()
            // BORROW: `read_line` takes a mutable reference `&mut guess`.
            // It borrows `guess` to write data into it, but does not take ownership.
            .read_line(&mut guess)
            .expect("Failed to read line");

        // OWNERSHIP: `guess.trim()` returns a `&str` (slice).
        // This `guess` variable (shadowing the previous one) is a reference borrowing from the String `guess`.
        // The original String `guess` still owns the data.
        let guess = guess.trim();

        match guess {
            "quit" => {
                println!("Quitting the game prematurely");
                break;
            }
            _ if !is_single_code_point(guess) => {
                println!("What you have was not a single character!");
                continue;
            }
            _ => {}
        }

        // OWNERSHIP: `guess.to_string()` creates a NEW String (a clone of the data).
        // `guessed_chars.push(...)` takes ownership of this new String and stores it in the vector.
        // The original `guess` String (from line 27) will be dropped at the end of this loop iteration.
        guessed_chars.push(guess.to_string());

        // OWNERSHIP: `join` creates a NEW String containing the result.
        // `guessed_chars` is borrowed (implicitly) to read the data, but ownership stays with `guessed_chars`.
        //let guessed_chars_string = guessed_chars.join(",");
        //println!("Your guesses: {guessed_chars_string}");

        if selected_word.contains(guess) {
            println!("Correct guess!");
        } else {
            guess_count_left = guess_count_left - 1;
            println!("Wrong guess!");
        }

        // OWNERSHIP: `selected_word.chars()` creates an iterator.
        // It borrows `selected_word` (which borrows from `words`).
        // No ownership is transferred here.

        let mut gathered_total_chars:usize = 0;
        for selected_char in selected_word.chars() {
            // `selected_char` is a `char` (Copy type). It is copied from the string.
            
            // OWNERSHIP: `selected_char.to_string()` creates a temporary String.
            // `guessed_chars.contains(...)` takes a reference to that temporary String.
            // The temporary String is dropped immediately after this line.
            if guessed_chars.contains(&selected_char.to_string()) {
                print!("{selected_char}");
                gathered_total_chars += 1;
            } else {
                print!("_");
            }
            
        }
        println!("");
        
        if total_chars - gathered_total_chars == 0 {
            break
        }

        match guess_count_left {
            0 => {
                println!("You lost!");
                process::exit(1);
            }
            1 => {
                println!("{guess_count_left} guess left");
            }
            _ => {
                println!("{guess_count_left} guesses left");
            }
        }
        
        // OWNERSHIP: `guess` (the String created at start of loop) goes out of scope here and is dropped (freed).
    }
    println!("You have won!")
}