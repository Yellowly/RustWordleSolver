use std::{
    collections::LinkedList,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    //my first line of rust code, previously said hello world but changed for release
    println!("Hello wordle");

    //stores remaining possabilities, each word is stored as an integer representing an index of the words array
    let mut possibilities: LinkedList<u16> = LinkedList::new();

    //stores every possible word from a text file as an array of 8 bit ints representing ascii chars because i dont understand how utf8 works
    let words: Vec<[u8;5]> = words_to_arr("5letterwords.txt");
    let posswords: Vec<[u8;5]> = words_to_arr("5letterhiddenwords.txt");
  println!("{}",posswords.len());
    //generates the remaining possibilities
    let mut pointer: usize = 0;
    for i in 0..words.len() {
        if pointer<posswords.len() && arr_eq(&words[i],&posswords[pointer]) {
           possibilities.push_back(i as u16);
          pointer+=1;
        }
    }
    let mut run: bool = true;

    println!("Wordle solver! 'end' to exit");

    //wall of text gl reading this
    while run {
        if possibilities.len() == 0 {
            println!("no remaining possibilities!");
            break;
        }
        let mut input = String::new();
        let guess: u16 = match possibilities.len() {
            2315 => 8845,
            _ => guess_word(&words, &possibilities).0,
        };
        println!("Guessing...: {0}", u8arr_to_string(&words[guess as usize]));
        println!("Pattern: ");
        io::stdin().read_line(&mut input).expect("error");
        input = input.trim().to_string();
        if input.eq("end") {
            run = false;
        } else if input.eq("poss") {
            println!(
                "{0} possabilities: {1:?}",
                possibilities.len(),
                possibilities
            );
        } else {
            possibilities = update_poss(&words, &possibilities, guess, get_pattern(&input));
        }
    }
    println!("ended!");
}
fn arr_eq(arr_1 : &[u8; 5], arr_2 : &[u8; 5]) -> bool{
  arr_1.iter().zip(arr_2.iter()).all(|(a,b)| a == b) 
}
//takes words from 5letterhiddenwords.txt and stores into array
fn words_to_arr(file_name: &str) -> Vec<[u8; 5]> {
    let mut res: Vec<[u8; 5]> = Vec::new();
    //let mut line_number: usize = 0;
    if let Ok(lines) = read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut hold: [u8;5] = [0;5];
                for (i, c) in ip.chars().enumerate() {
                    hold[i] = (c as u8) - 97;
                }
                res.push(hold);
                //line_number += 1;
            }
        }
    } else {
        println!("error! ");
    }
    res
}

//turns an array of u8's to their ascii char representation
fn u8arr_to_string(ints: &[u8]) -> String {
    let mut res: String = String::from("");
    for n in ints {
        res.push((n + 97) as char);
    }
    res
}

// Returns an Iterator to the Reader of the lines of the file. copy pasted from rust website lol
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
//used to update the remaining number of possible words
fn update_poss(
    words: &Vec<[u8;5]>,
    poss: &LinkedList<u16>,
    guessed_word: u16,
    pattern: u8,
) -> LinkedList<u16> {
    let mut new_poss: LinkedList<u16> = LinkedList::new();

    //poss.drain_filter(|w| gen_pattern(&words[guessedWord as usize], &words[*w as usize]) == pattern); this function would be so useful but sadly it's only on nightly rust release

    for &w in poss {
        if gen_pattern(&words[guessed_word as usize], &words[w as usize]) == pattern {
            new_poss.push_back(w);
        }
    }
    new_poss
}
//evaluates a word using information theory formula thing
fn evaluate(words: &Vec<[u8;5]>, guess: u16, poss: &LinkedList<u16>) -> f32 {
    let mut num_of_poss: [u16; 243] = [0; 243];
    //stores number of possibilities of a word given a pattern, for example [0] stores all possible words if pattern was Gray Gray Gray Gray Gray
    for &i in poss {
        let pattern: u8 = gen_pattern(&words[guess as usize], &words[i as usize]);
        num_of_poss[pattern as usize] += 1;
    }
    //actually does the information theory math
    let mut res: f32 = 0.0;
    for i in num_of_poss {
        let prob: f32 = (i as f32) / (poss.len() as f32);
        if prob > 0.00000001 {
            res += prob * f32::log2(1.0 / prob);
        }
    }
    res
}
//guesses a word by finding the word with best eval
fn guess_word(words: &Vec<[u8;5]>, poss: &LinkedList<u16>) -> (u16, f32) {
    let mut best_eval: f32 = 0.0;
    let mut best_found: u16 = 0;

    //random number i made up, if more than 1 remaining then finds the word with best eval, otherwise find POSSIBLE word with best eval
    if poss.len() > 2 {
        for i in 0..2315 {
            let eval: f32 = evaluate(&words, i, &poss);
            if eval >= best_eval {
                best_eval = eval;
                best_found = i;
            }
        }
    } else {
        for &i in poss {
            //ctrl c ctrl v lmao too lazy to make another method
            let eval: f32 = evaluate(&words, i, &poss);
            if eval >= best_eval {
                best_eval = eval;
                best_found = i;
            }
        }
    }
    (best_found, best_eval)
}
//generates a pattern given a guess and answer. patterns are base-3 numbers where 0 is gray, 1 is yellow, and 2 is green
//dont question how this works, i trust 3 weeks ago me to know what he was doing
fn gen_pattern(guess: &[u8], answer: &[u8]) -> u8 {
    let mut res: u8 = 0;
    let mut chars_in_guess: [u8; 26] = [0; 26];
    let mut chars_in_answer: [u8; 26] = [0; 26];
    for i in (0..5).rev() {
        res *= 3;
        if guess[i] == answer[i] {
            res += 2;
        }
        chars_in_guess[guess[i] as usize] += 1;
        chars_in_answer[answer[i] as usize] += 1;
    }
    let mut yellows: u8 = 0;
    for i in (0..5).rev() {
        yellows *= 3;
        if guess[i] != answer[i] {
            if chars_in_answer[guess[i] as usize] < chars_in_guess[guess[i] as usize] {
                chars_in_guess[guess[i] as usize] -= 1;
            } else {
                yellows += 1;
            }
        }
    }
    res + yellows
}
//turns a inputted pattern, for example YG-Y- for yellow green gray yellow gray, into the base-3 pattern representation
fn get_pattern(input: &String) -> u8 {
    let mut res: u8 = 0;
    for c in input.chars().rev() {
        res *= 3_u8;
        res += match c {
            'G' => 2_u8,
            'Y' => 1_u8,
            _ => 0_u8,
        }
    }
    res
}
