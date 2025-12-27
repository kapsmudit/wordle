use std::env;
use colored::{ColoredString, Colorize};
use rand::prelude::IndexedRandom;
use wordle::Correctness;

const ANSWERS: &str = include_str!("../answers.txt");
const DICTIONARY: &str = include_str!("../dictionary.txt");
const GUESS_LENGTH: usize = 5;
const GUESSES: usize = 6;

fn play(answer: &str) {
    let dictionary: Vec<&str> = DICTIONARY.lines().collect();
    let mut tries = GUESSES;
    while tries > 0 {
        println!("\n{}>", tries-1);
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess = guess.trim().to_string();
        if guess.len() != GUESS_LENGTH {
            println!("\nGuess should be {GUESS_LENGTH} characters long.");
            continue;
        } else if !dictionary.contains(&guess.as_str()) {
            println!("\nNot a valid word");
            continue;
        }
        tries -= 1;
        let result = Correctness::check(&answer, &guess);
        for (guess, result) in guess.chars().zip(result) {
            let out = match result {
                Correctness::Correct => ColoredString::from(String::from(guess)).black().on_bright_green().bold(),
                Correctness::Misplaced => ColoredString::from(String::from(guess)).black().on_bright_yellow().bold(),
                Correctness::Incorrect => ColoredString::from(String::from(guess)).white().on_black().bold()
            };
            print!("{}", out);
        }
        if result.iter().all(|x| { *x == Correctness::Correct }) {
            println!("\nYou won!");
            return;
        }
    }
    println!("\nYou lost.");
    println!("Answer is {}", answer);
}

fn draw(answer: &str, patterns: &Vec<[Correctness; 5]>) {
    let dictionary: Vec<&str> = DICTIONARY.lines().collect();
    assert_eq!(answer.len(), GUESS_LENGTH);
    for pattern in patterns {
        let found = dictionary.iter().find(|&&word| Correctness::check(answer, word) == *pattern).unwrap_or(&"NOTFOUND");
        println!("{}", *found);
    }
}


fn play_draw() {
    let mut answer = String::new();
    println!("Today's answer: ");
    std::io::stdin().read_line(&mut answer).expect("Failed to read line");
    let patterns: Vec<[Correctness; GUESS_LENGTH]> = include_str!("../pattern").trim().lines().map(|line| {
        let mut correctness: [Correctness; GUESS_LENGTH] = [Correctness::Incorrect; GUESS_LENGTH];
        assert_eq!(line.len(), GUESS_LENGTH);
        for (i, c) in line.chars().enumerate() {
            correctness[i] = match c {
                'x' => Correctness::Correct,
                'o' => Correctness::Misplaced,
                '.' => Correctness::Incorrect,
                _ => panic!()
            }
        }
        correctness
    }).collect();
    assert_eq!(patterns.len(), GUESSES);
    patterns.iter().for_each(|&pattern| assert_eq!(pattern.len(), GUESS_LENGTH));

    draw(&answer.trim().to_lowercase(), &patterns);
}

fn main() {
    let answers: Vec<&str> = ANSWERS.lines().collect();
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str){
        Some("play") => play(answers.choose(&mut rand::rng()).unwrap()),
        Some("draw") => play_draw(),
        _ => play(answers.choose(&mut rand::rng()).unwrap())
    }
}
