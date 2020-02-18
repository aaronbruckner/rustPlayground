use std::io::{self, Read};
use std::str::Chars;
use std::iter::Peekable;

/*
 * Converts standard in to pig-latin. Handles unicode characters. Leaves all other spacing and punctuation
 * untouched.
 * 
 * To run: 
 *      cargo run --bin pig_latin
 */ 
fn main() {
    let mut raw_string: String = String::new();
    io::stdin().read_to_string(&mut raw_string)
        .expect("Failure while reading from stdin");

    let mut pig_string: String = String::new();

    let mut chars: Peekable<Chars> = raw_string.chars().peekable();

    while chars.peek().is_some() {
        if chars.peek().unwrap().is_alphabetic() {
            let mut word = extract_word(&mut chars);
            pigify_word(&mut word);
            pig_string.push_str(& word);
        } else {
            pig_string.push(chars.next().unwrap());
        }
    }
    println!("{}", pig_string);
}

fn extract_word(chars: &mut Peekable<Chars>) -> String {
    let mut word: String = String::new();
    while chars.peek().unwrap_or(& '_').is_alphabetic() {
        word.push(chars.next().unwrap());
    }
    word
}

fn pigify_word(word: &mut String) {
    let first_char: char = word.chars().next().unwrap();
    if is_vowel(first_char) {
        word.push_str("-hay");
    } else {
        word.remove(0);
        word.push_str(& format!("-{}ay", first_char));
    }
}

fn is_vowel(c: char) -> bool {
    match c.to_lowercase().next().unwrap() {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false
    }
}