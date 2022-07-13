use std::io;
use std::fs;

use rand::{prelude::SliceRandom, thread_rng};

fn main() {
    // Loading words
    let words = fs::read_to_string("./words.txt") // Returns Result<String>
        .expect("Could not read file") // Returns String
        .split("\n") // Returns Split<&str>, which Implements Iterator
        .map(String::from) // Returns Map<Split<&str>, (&str) -> String>
        .collect::<Vec<String>>();

    
    println!("Hangman!");
    let mut done = run_game(&words);
    while !done {
        done = run_game(&words);
    }

}

fn print_board(lives: &u8, display: &Vec<String>) {
    let man =  vec!["O", "|", "/", "\\", "/", "\\"];
    let mut temp = vec![" ", " ", " ", " ", " ", " "];

    for i in 0..(6 - lives) {
        temp[i as usize] = &man[i as usize];
    }

    println!("----------");
    println!("         |");
    println!("         {}", &temp[0]);
    println!("        {}{}{}", &temp[2], &temp[1], &temp[3]);
    println!("        {} {}", &temp[4], &temp[5]);
    println!();
    println!("{}", display.join(" "));
}

fn run_game(words: &Vec<String>) -> bool {
    let answer = words.choose(&mut thread_rng())
    .expect("No word chosen!").split("").collect::<Vec<&str>>();

    let mut display = Vec::<String>::new();
    let mut wrong_letters = Vec::<String>::new();

    let answer_length = answer.join("").replace("\r", "").len();

    for _ in 0..answer_length {
        display.push("_".to_string());
    }
    let mut done = false;
    let mut lives: u8 = 6;
    println!("The secret words's length is {:?}", &answer_length);
    println!("Input your guess below.");

    while !&done && &lives > &0 {
        print_board(&lives, &display);
        println!("Wrong letters: [{}]", wrong_letters.join(", "));

        // Read Input from terminal
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");
        guess = guess.replace("\n", "").replace("\r", "");
        println!("Guessed: {:?}", guess);
        // Check guess
        if &guess == &answer.join("").replace("\r", "") {
            println!("Correct!");
            done = true;
        }
        else {
            let mut contain = false;
            for i in 0..answer_length {
                if &answer[i + 1] == &guess {
                    contain = true;
                    display[i] = guess.clone();
                }
            }
            if contain {
                println!("Correct!");
                for i in &display {
                    if i == "_" {
                        done = false;
                        break;
                    }
                    else {
                        done = true;
                    }
                }
            }
            else {
                println!("Incorrect!");
                lives -= 1;
                wrong_letters.push(guess.clone());
            }
        }
        println!("Press enter to continue...");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");
        print!("\x1B[2J\x1B[1;1H");
    }
    if done {
        for i in 0..display.len() {
            display[i] = answer[i + 1].to_string();
        }
        print_board(&lives, &display);
        println!("You win!");
        println!("Play again? Enter 'Y' or 'N'");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");
        while guess.replace("\r", "").replace("\n", "") != "Y" && guess.replace("\r", "").replace("\n", "") != "N" {
            println!("Didn't get that! Enter 'Y' or 'N'");
            guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read input.");
        }
        return guess.replace("\r", "").replace("\n", "") == "N";
    }
    print_board(&lives, &display);
    println!("You lose! The word was '{}'.", &answer.join("").replace("\r", ""));
    println!("Play again? Enter 'Y' or 'N'");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input.");
    while guess.replace("\r", "").replace("\n", "") != "Y" && guess.replace("\r", "").replace("\n", "") != "N" {
        println!("Didn't get that! Enter 'Y' or 'N'");
        guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");
    }
    return guess.replace("\r", "").replace("\n", "") == "N";
}