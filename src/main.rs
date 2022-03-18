#![feature(portable_simd)]

mod guess_matcher;
mod word;
mod test;


use std::fs;
use std::simd::*;
use rayon::prelude::*;
use crate::guess_matcher::GuessMatcher;
use crate::word::*;

fn best_guess(possible_guesses: &[u8x8], possible_answers: &[u8x8], filters: &[GuessMatcher]) -> Vec<u8x8> {
    let answers_count: Vec<u8x32> = possible_answers.iter().map(|x|to_count(*x)).collect();

    let pre_filtered : Vec<bool> = possible_answers.iter().zip(answers_count.iter())
        .map(|(&ans, &ans_count)|
            filters.iter().all(|f|
                f.matches_fast(ans, ans_count)
            )).collect();

    let avg_matched: Vec<usize> = possible_guesses.par_iter().map(
        //for every potential guess
        |&word| {

            //for every possible answer
            let others_matched = possible_answers.iter()
                .map(|&ans| {
                    for matcher in filters {
                        if !matcher.matches(ans) {
                            return 0;
                        }
                    }
                    let matcher = GuessMatcher::from_guess(word, ans);

                    possible_answers.iter().zip(answers_count.iter()).zip(pre_filtered.iter())
                        .filter(|(_,&c)| c)
                        .filter(
                            |((&ans, &ans_count), _)|
                                matcher.matches_fast(ans, ans_count)
                        ).count()
                })
                .sum();

            if filters.iter().all(|f|f.matches(word)) {
                return others_matched - 1;
            } else {
                return others_matched;
            }
        }
    ).collect();


    let mut best_pairs: Vec<(usize, u8x8)> = avg_matched.iter().zip(possible_guesses).map(|(&a,&b)|(a,b)).collect();
    best_pairs.sort_by_key(|(a,b)| *a);
    return best_pairs.iter().map(|(a,b)| *b).collect();
    // min_by_key(|(&count, _)| count).expect("No possible guesses");
    // return *best_guess;
}

fn main() {
    let answers_str = fs::read_to_string("data/Answers.txt").expect("Couldn't open answers.txt");
    let guesses_str = fs::read_to_string("data/Guesses.txt").expect("Couldn't open guesses.txt");
    let answers = split(answers_str.as_str());
    let guesses = split(guesses_str.as_str());
    let all_words : Vec<u8x8> = answers.iter().chain(guesses.iter()).map(|&x|x).collect();

    let start = "roate";
    println!("{}", start);
    let mut previous_word = to_word(start);
    let mut existing_filters: Vec<GuessMatcher> = Vec::new();

    // existing_filters.push(GuessMatcher::from_result(to_word("other"), "00000").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("tenor"), "00000").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("facet"), "00000").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("matey"), "00000").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("final"), "01002").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("waist"), "00100").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("gleam"), "01000").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("sleet"), "01000").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("email"), "00022").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("stake"), "00000").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("exile"), "00110").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("alpha"), "01200").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("apnea"), "01000").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("speck"), "01000").unwrap());
    // existing_filters.push(GuessMatcher::from_result(to_word("belle"), "00100").unwrap());

    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Error reading from stdin");
        if let Some(stripped) = buffer.strip_suffix('\n') {
            if stripped == "win" {
                println!("{}", start);
                previous_word = to_word(start);
                existing_filters.clear();
                continue;
            }
            if let Some(matcher) = GuessMatcher::from_result(previous_word, stripped) {
                existing_filters.push(matcher);

                let best = best_guess(all_words.as_slice(), answers.as_slice(), &existing_filters);

                // let best = if(existing_filters.len() > 1){
                // best_guess(all_words.as_slice(), all_words.as_slice(), &existing_filters)
                // } else {
                //     best_guess(all_words.as_slice(), answers.as_slice(), &existing_filters)
                // };

                println!("{},{},{},{},{}", u8_to_string(best[0]),u8_to_string(best[1]),u8_to_string(best[2]),u8_to_string(best[3]),u8_to_string(best[4]));
                let mut b = String::new();
                std::io::stdin().read_line(&mut b).expect("err reading stdio");
                b.pop();
                if b == "win" {
                    println!("{}", start);
                    previous_word = to_word(start);
                    existing_filters.clear();
                    continue;
                }
                let idx: usize = b.parse().expect("parse err");
                previous_word = best[idx];
            } else {
                println!("Error reading input, bad format");
                continue;
            }
        } else {
            println!("Error reading input, couldn't read from stdin");
            continue;
        }
    }
}

fn u8_to_string(word: u8x8) -> String {
    let mut chars = [0;5];
    for i in 0..5 {
        chars[i] = word[i];
    }
    return std::str::from_utf8(&chars).unwrap().into();
}