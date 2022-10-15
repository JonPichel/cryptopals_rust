use std::collections::HashMap;

fn single_byte_xor_decrypt_by_score(bytes: &[u8]) -> String {
    let mut best_cleartext: String = "".to_string();
    let mut best_score = 0f32;
    for key in 0..=255 {
        let mut cleartext = bytes.clone().to_vec();
        xor_bytes_to_single(&mut cleartext, key);
        let cleartext = String::from_utf8(cleartext);
        if cleartext.is_err() {
            continue;
        }
        let cleartext = cleartext.unwrap();

        let score = score_text(&cleartext);
        dbg!(&score, &cleartext);

        if score > best_score {
            best_score = score;
            best_cleartext = cleartext;
        }
    }
    best_cleartext
}

fn score_text(text: &str) -> f32 {
    // We don't evaluate non ascii texts
    if !text.is_ascii() {
        return 0f32;
    }

    let num_chars = text.len();

    // More letters -> GOOD
    let mut char_score = 0;
    for char in text.chars() {
        if char.is_ascii_alphabetic() {
            char_score += 100;
        } else if char.is_ascii_whitespace() {
            char_score += 80;
        } else if char.is_ascii_punctuation() {
            char_score += 50;
        } else if char.is_numeric() {
            char_score += 30;
        }
    }
    let char_score = char_score as f32 / num_chars as f32;

    // Adjusts letter frequencies -> GOOD
    let counts = letter_count(text);
    let mut freq_score = 0;
    let top_chars = "etaoin";
    let worst_chars = "zqxjkv";


    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by_key(|item| item.1);
    for (index, item) in sorted[..6].iter().enumerate() {
        if *item.0 == top_chars.as_bytes()[index] as char {
            freq_score += 1;
        }
    }

    sorted.reverse();
    for (index, item) in sorted[..6].iter().enumerate() {
        if *item.0 == worst_chars.as_bytes()[index] as char {
            freq_score += 1;
        }
    }
    return char_score + freq_score as f32;
}

fn letter_count(text: &str) -> HashMap<char, usize> {
    let mut sorted: Vec<char> = text.to_ascii_lowercase().chars().collect();
    sorted.sort_by(|a, b| b.cmp(a));

    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut current = sorted[0];
    let mut count: usize = 0;
    for byte in sorted.iter() {
        if *byte != current {
            if current.is_ascii_alphabetic() {
                counts.insert(current as char, count);
            }
            current = *byte;
            count = 0;
        }
        count += 1;
    }
    if current.is_ascii_alphabetic() {
        counts.insert(current as char, count);
    }
    for letter in 'a'..='z' {
        if !counts.contains_key(&letter) {
            counts.insert(letter, 0);
        }
    }

    counts
}

fn xor_bytes_to_single(bytes: &mut Vec<u8>, key: u8) {
    for byte in bytes.iter_mut() {
        *byte ^= key;
    }
}

#[cfg(test)]
mod tests {
    use crate::set_one::exercise_one::hex_to_bytes;
    use super::*;

    #[test]
    fn test_single_byte_xor_decrypt_frequency() {
        let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let ciphertext = hex_to_bytes(ciphertext).unwrap();
        let cleartext = single_byte_xor_decrypt_by_score(&ciphertext);
        dbg!(&cleartext);
        assert_eq!(cleartext, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn test_letter_count() {
        let text = "hello WORLD";
        let mut expected: HashMap<char, usize> = HashMap::new();
        expected.insert('h', 1);
        expected.insert('e', 1);
        expected.insert('l', 3);
        expected.insert('o', 2);
        expected.insert('w', 1);
        expected.insert('r', 1);
        expected.insert('d', 1);
        for letter in 'a'..='z' {
            if !expected.contains_key(&letter) {
                expected.insert(letter, 0);
            }
        }

        let actual = letter_count(text);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_xor_bytes_to_single() {
        let mut bytes: Vec<u8> = vec![0b11111111, 0b00000000, 0b10101010, 0b01010101];
        let key = 0b10101010;
        xor_bytes_to_single(&mut bytes, key);
        assert_eq!(bytes, vec![0b01010101, 0b10101010, 0b00000000, 0b11111111]);
    }
}