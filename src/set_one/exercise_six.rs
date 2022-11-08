use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::set_one::error::SetOneError;
use crate::set_one::exercise_three::single_character_xor_decrypt_by_score;

pub fn base64_to_bytes(base64: &str) -> Result<Vec<u8>, SetOneError> {
    // TODO: check if padding is alright
    if !base64.is_ascii() {
        return Err(SetOneError::InvalidBase64(base64.to_string()));
    }

    let mut byte: u8 = 0;
    let mut bits_left_value: u8;
    let mut bits_left_byte: u8 = 8;
    let mut bytes: Vec<u8> = Vec::with_capacity(3 * base64.len() / 4);
    for char in base64.bytes() {
        bits_left_value = 6;
        // A..=Z -> 0..=25
        // a..=z -> 26..=51
        // 0..=9 -> 52..=61
        // + -> 62, / -> 63
        let value = match char {
            b'A'..=b'Z' => char - b'A',
            b'a'..=b'z' => char - b'a' + 26,
            b'0'..=b'9' => char - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => return Err(SetOneError::InvalidBase64(base64.to_string()))
        };

        while bits_left_value > 0 {
            if bits_left_byte == 0 {
                bits_left_byte = 8;
            }

            //   aaaaaabb bbbbcccc ccdddddd
            // B 8     2  8   4    8 6
            // V 6     6  4   6    2 6
            if bits_left_byte <= bits_left_value {
                // Will complete the byte
                byte |= value >> (6 - bits_left_byte);
                bits_left_value -= bits_left_byte;
                bits_left_byte = 0;
                bytes.push(byte);
                byte = 0;
            } else {
                // Won't complete the byte
                byte |= value << (8 - bits_left_value);
                bits_left_byte -= bits_left_value;
                bits_left_value = 0;
            }
        }
    }

    Ok(bytes)
}

pub fn exercise_six(path: impl AsRef<Path>) {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let joined = lines.fold("".to_string(), |acc, x| {
        format!("{}{}", acc, x.unwrap())
    });

    let bytes = base64_to_bytes(&joined).unwrap();

    if let Some(keysizes) = guess_keysize(&bytes, 2, 40, 4) {
        for keysize in keysizes {
            let (cleartext, key) = break_repeating_key_xor(bytes.as_ref(), keysize);
            println!("KEYSIZE={} key=\"{}\"", &keysize, &String::from_utf8(key).unwrap());
            println!("{}", &cleartext);
            if cleartext.is_ascii() {
                break;
            }
        }
    }

}

pub fn break_repeating_key_xor(bytes: &[u8], keysize: usize) -> (String, Vec<u8>) {
    let mut transposed: Vec<Vec<u8>> = vec![Vec::with_capacity(bytes.len() / keysize); keysize];
    let mut key: Vec<u8> = Vec::with_capacity(keysize);
    for chunk in bytes.chunks(keysize) {
        for i in 0..chunk.len() {
            transposed[i].push(chunk[i]);
        }
    }

    let mut transposed_strings: Vec<String> = Vec::with_capacity(keysize);
    for subarray in transposed {
        let best_substring = single_character_xor_decrypt_by_score(&subarray);
        transposed_strings.push(best_substring.cleartext);
        key.push(best_substring.key);
    }

    let mut cleartext = String::with_capacity(bytes.len());

    for _ in 0..transposed_strings[0].len() - 1 {
        for substring in transposed_strings.iter_mut() {
            cleartext.push(substring.remove(0));
        }
    }

    for string in transposed_strings {
        if !string.is_empty() {
            cleartext.push_str(&string);
        }
    }

    (cleartext, key)
}

pub fn guess_keysize(bytes: &[u8], min_len: usize, max_len: usize, n_blocks: usize) -> Option<Vec<usize>> {
    if n_blocks * max_len > bytes.len() {
        return None;
    }

    let mut distances = HashMap::new();
    let num_distances: usize = (0..n_blocks).sum();
    for keysize in min_len..=max_len {
        let mut blocks: Vec<&[u8]> = Vec::with_capacity(n_blocks);
        for i in 0..n_blocks {
            blocks.push(&bytes[i*keysize..(i+1)*keysize]);
        }

        let mut edit_distance = 0;
        for i in 0..(n_blocks-1) {
            for j in (i+1)..n_blocks {
                edit_distance += hamming_distance(blocks[i], blocks[j]).unwrap();
            }
        }

        let edit_distance = edit_distance as f32 / (keysize * num_distances) as f32;
        distances.insert(keysize, edit_distance);
    }

    let mut keysizes: Vec<usize> = distances.keys().cloned().collect();
    keysizes.sort_by(|a, b| {
        distances.get(a).unwrap().partial_cmp(distances.get(b).unwrap()).unwrap_or(Equal)
    });

    Some(keysizes)
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> Option<usize> {
    if a.len() != b.len() {
        return None;
    }

    let mut distance = 0_usize;

    for (ba, bb) in a.iter().zip(b) {
        let diff = *ba ^ *bb;
        for i in 0..8 {
            distance += ((diff >> i) & 0b1) as usize;
        }
    }
    Some(distance)

}

#[cfg(test)]
mod tests {
    use crate::set_one::exercise_one::hex_to_bytes;
    use super::*;

    #[test]
    fn test_base64_to_bytes() {
        let base64 = "FCg8UGR4jA==";
        let expected: [u8; 7] = [20, 40, 60, 80, 100, 120, 140];
        assert_eq!(base64_to_bytes(base64).unwrap(), expected);

        let base64 = "aG9sYSBxdWUgdGFs";
        let expected: Vec<u8> = "hola que tal".bytes().collect();
        assert_eq!(base64_to_bytes(base64).unwrap(), expected);
    }

    #[test]
    fn test_hamming_distance() {
        let a = "this is a test".to_string().as_bytes().to_vec();
        let b = "wokka wokka!!!".to_string().as_bytes().to_vec();

        assert_eq!(hamming_distance(&a, &b).unwrap(), 37);
    }

    #[test]
    fn test_guess_keysize() {
        // TODO: come up with a test for this function
        let _key = "ICE";
        let ciphertext = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let bytes = hex_to_bytes(ciphertext).unwrap();
        guess_keysize(&bytes, 2, 5, 6);
        assert!(true);
    }
}