use std::simd::*;
use crate::word::to_count;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct GuessMatcher {
    wrong_letters: u8x8,
    right_letters: u8x8,
    min_counts : u8x32,
    max_counts : u8x32,
}

impl GuessMatcher {
    pub fn from_result(guess: u8x8, mask: &str) -> Option<GuessMatcher> {
        let mask = mask.as_bytes();

        if mask.len() != 5 { return None; }
        if !mask.iter().all(|&x| x == '0' as u8 || x == '1' as u8 || x == '2' as u8) { return None; }

        let counts = to_count(guess);
        let mut wrong_letters = [1u8;8];
        let mut right_letters = [0u8;8];

        for i in 0..5 {
            if mask[i] == '0' as u8 || mask[i] == '1' as u8 {
                wrong_letters[i] = guess[i];
            } else {
                right_letters[i] = guess[i];
            }
        }


        let mut mins = [0u8;32];
        let mut maxes = [5u8;32];

        for i in 0..32 {
            if counts[i] != 0 {
                let total = guess.to_array().iter().filter(|&x| *x != 0 && x - 'a' as u8 == i as u8).count() as u8;
                let right = guess.to_array().iter().zip(mask).filter(|&(&x,&m)| x != 0 && x - 'a' as u8 == i as u8 && m != '0' as u8).count() as u8;

                if total > right {
                    mins[i] = right;
                    maxes[i] = right;
                } else {
                    mins[i] = total;
                    maxes[i] = 5;
                }
            }
        }

        return Some(GuessMatcher{
            wrong_letters: u8x8::from(wrong_letters),
            right_letters: u8x8::from(right_letters),
            min_counts: u8x32::from(mins),
            max_counts: u8x32::from(maxes)
        })
    }
    pub fn from_guess(guess: u8x8, correct: u8x8) -> GuessMatcher {
        let mut locations = u8x8::default();
        let mut wrong_letters = [1;8];
        for i in 0..5 {
            if guess[i] == correct[i] {
                locations[i] = guess[i];
            } else {
                wrong_letters[i] = guess[i];
            }
        }

        let wrong_letters = u8x8::from(wrong_letters);


        let guess_count = to_count(guess);
        let correct_count = to_count(correct);
        let mut min_counts = u8x32::default();
        let mut max_counts = u8x32::default();
        for idx in 0..32 {
            let guess = guess_count[idx];
            let answer = correct_count[idx];
            if guess <= answer {
                min_counts[idx] = guess;
                max_counts[idx] = 5;
            } else {
                min_counts[idx] = answer;
                max_counts[idx] = answer;
            }
        }

        return GuessMatcher{min_counts, max_counts, right_letters: locations, wrong_letters };
    }
    pub fn matches(&self, word: u8x8) -> bool {
        let counts = to_count(word);
        return self.matches_fast(word, counts);
    }
    pub fn matches_fast(&self, word: u8x8, counts: u8x32) -> bool {

        if !counts.lanes_ge(self.min_counts).all()
            | !counts.lanes_le(self.max_counts).all() {
            return false;
        }

        if self.wrong_letters.lanes_eq(word).any() {
            return false;
        }

        let zeros = u8x8::default();
        let or = self.right_letters.lanes_eq(word) | self.right_letters.lanes_eq(zeros);
        if !or.all() {
            return false;
        }

        return true;
    }
}