use std::collections::HashMap;

pub struct SingleCharacterXorCleartext {
    pub cleartext: String,
    pub key: u8,
    pub score: f32,
}

pub fn single_character_xor_decrypt_by_score(bytes: &[u8]) -> SingleCharacterXorCleartext {
    let mut best = SingleCharacterXorCleartext {
        cleartext: "".to_string(),
        key: 0,
        score: f32::MAX,
    };
    for key in 0..=255 {
        let mut cleartext = bytes.to_vec();
        single_character_xor(&mut cleartext, key);

        let cleartext = String::from_utf8(cleartext);
        if cleartext.is_err() {
            continue;
        }

        let cleartext = cleartext.unwrap();

        let score = score_text(&cleartext);

        if score < best.score {
            best.score = score;
            best.key = key;
            best.cleartext = cleartext;
        }
    }
    best
}

pub fn score_text(text: &str) -> f32 {
    // We don't evaluate non ascii texts
    if !text.is_ascii() {
        return f32::MAX;
    }
    let text = text.to_ascii_uppercase();

    // Adjusts letter frequencies -> GOOD
    score_text_by_letter_frequencies(&text)
}

fn score_text_by_letter_frequencies(text: &str) -> f32 {
    let expected: HashMap<char, f32> = [
        (' ', 12.17), // Whitespace
        ('.', 6.57),  // Others
        ('a', 6.09),
        ('b', 1.05),
        ('c', 2.84),
        ('d', 2.92),
        ('e', 11.36),
        ('f', 1.79),
        ('g', 1.38),
        ('h', 3.41),
        ('i', 5.44),
        ('j', 0.24),
        ('k', 0.41),
        ('l', 2.92),
        ('m', 2.76),
        ('n', 5.44),
        ('o', 6.00),
        ('p', 1.95),
        ('q', 0.24),
        ('r', 4.95),
        ('s', 5.68),
        ('t', 8.03),
        ('u', 2.43),
        ('v', 0.97),
        ('w', 1.38),
        ('x', 0.24),
        ('y', 1.30),
        ('z', 0.03),
    ].iter().cloned().collect();
    let observed = letter_frequencies(text);

    let mut score = 0.0;
    for (char, exp) in expected {
        let obs = match observed.get(&char) {
            Some(x) => *x,
            None => 0.0,
        };
        score += (obs - exp).abs();
    }
    score
}

pub fn letter_frequencies(text: &str) -> HashMap<char, f32> {
    let len = text.len() as f32;

    let mut counts: HashMap<char, f32> = HashMap::new();
    for byte in text.chars() {
        match byte {
            'A'..='Z' => *counts.entry(byte.to_ascii_lowercase()).or_insert(0.0) += 1.0,
            'a'..='z' => *counts.entry(byte).or_insert(0.0) += 1.0,
            ' ' | '\t' => *counts.entry(' ').or_insert(0.0) += 1.0,
            _ => *counts.entry('.').or_insert(0.0) += 1.0,
        }
    }

    // Normalize by the text length
    counts.iter_mut().for_each(|item| *(item.1) *= 100.0 / len);
    counts
}

pub fn single_character_xor(bytes: &mut [u8], key: u8) {
    for byte in bytes.iter_mut() {
        *byte ^= &key;
    }
}

#[cfg(test)]
mod tests {
    use crate::set_one::exercise_one::hex_to_bytes;
    use super::*;

    #[test]
    fn test_single_character_xor_decrypt_frequency() {
        let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let ciphertext = hex_to_bytes(ciphertext).unwrap();
        let best = single_character_xor_decrypt_by_score(&ciphertext);
        assert_eq!(best.cleartext, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn test_letter_frequencies() {
        let text = "hello WORLD!";
        let len = text.len() as f32;
        let expected: HashMap<char, f32> = [
            ('h', 1.0 * 100.0 / len),
            ('e', 1.0 * 100.0 / len),
            ('l', 3.0 * 100.0 / len),
            ('o', 2.0 * 100.0 / len),
            (' ', 1.0 * 100.0 / len),
            ('.', 1.0 * 100.0 / len),
            ('w', 1.0 * 100.0 / len),
            ('r', 1.0 * 100.0 / len),
            ('d', 1.0 * 100.0 / len),
        ].iter().cloned().collect();

        let actual = letter_frequencies(text);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_single_character_xor() {
        let mut bytes: Vec<u8> = vec![0b11111111, 0b00000000, 0b10101010, 0b01010101];
        let key = 0b10101010;
        single_character_xor(&mut bytes, key);
        assert_eq!(bytes, vec![0b01010101, 0b10101010, 0b00000000, 0b11111111]);
    }
}