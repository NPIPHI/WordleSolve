use std::simd::*;

pub fn to_count(word: u8x8) -> u8x32 {
    let mut count: [u8;32] = [0;32];
    for i in 0..5 {
        let letter = word[i];
        unsafe { *count.get_unchecked_mut((letter - 'a' as u8) as usize) += 1; }
        // count[(letter - 'a' as u8) as usize] += 1;
    }
    return u8x32::from(count);
}

pub fn split(s: &str) -> Vec<u8x8> {
    s.split(',')
        .map(|s| {
            let mut arr = u8x8::default();
            for i in 0..5 {
                arr[i] = s.as_bytes()[i+1];
            }
            return arr;
        })
        .collect()
}

pub fn to_word(s: &str) -> u8x8 {
    let mut word = u8x8::default();
    for i in 0..5 {
        word[i] = s.as_bytes()[i];
    }
    return word;
}
