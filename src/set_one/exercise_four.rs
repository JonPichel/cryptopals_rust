use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::set_one::exercise_one::hex_to_bytes;
use crate::set_one::exercise_three::{single_character_xor_decrypt_by_score, SingleCharacterXorCleartext};

pub fn exercise_four(path: impl AsRef<Path>) {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();
    let mut bytes_array: Vec<Vec<u8>> = Vec::new();
    for line in lines {
        let bytes = hex_to_bytes(&line.unwrap()[..]).unwrap();
        bytes_array.push(bytes);
    }

    let best = detect_single_character_xor(bytes_array.as_ref());
    println!("cleartext={} score={}", &best.cleartext, &best.score);
}

pub fn detect_single_character_xor(bytes_array: &[Vec<u8>]) -> SingleCharacterXorCleartext {
    let mut best = SingleCharacterXorCleartext {
        cleartext: "".to_string(),
        key: 0,
        score: f32::MAX,
    };
    for bytes in bytes_array {
        let option = single_character_xor_decrypt_by_score(&bytes[..]);
        if option.score < best.score {
            best.score = option.score;
            best.key = option.key;
            best.cleartext = option.cleartext;
        }
    }
    best
}