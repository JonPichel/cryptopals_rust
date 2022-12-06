use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use openssl::symm::{Cipher, decrypt};
use crate::set_one::exercise_six::base64_to_bytes;

pub fn exercise_seven(path: impl AsRef<Path>) {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let joined = lines.fold("".to_string(), |acc, x| {
        format!("{}{}", acc, x.unwrap())
    });

    let bytes = base64_to_bytes(&joined).unwrap();

    let key: Vec<u8> = "YELLOW SUBMARINE".bytes().collect();

    let cleartext = decrypt(
        Cipher::aes_128_ecb(),
        &key,
        None,
        &bytes
    ).unwrap();

    println!("{}", String::from_utf8(cleartext).unwrap());
}