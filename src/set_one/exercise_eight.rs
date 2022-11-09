use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::set_one::exercise_one::hex_to_bytes;

pub fn exercise_eight(path: impl AsRef<Path>) {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let bytes_array: Vec<Vec<u8>> = lines.map(|line| {
        hex_to_bytes(&line.unwrap()).unwrap()
    }).collect();

    for (i, ciphertext) in bytes_array.iter().enumerate() {
        let mut repeats: HashMap<[u8; 16], usize> = HashMap::new();
        for block in ciphertext.chunks(16) {
            let block: [u8; 16] = block.try_into().unwrap();
            if repeats.contains_key(&block) {
                *repeats.get_mut(&block).unwrap() += 1;
            } else {
                repeats.insert(block, 0);
            }
        }
        let repeats: usize = repeats.values().sum();
        if repeats > 0 {
            println!("Line {} has {} repeated blocks. It was likely encrypted in ECB.", &i+1, &repeats);
        }
    }
}