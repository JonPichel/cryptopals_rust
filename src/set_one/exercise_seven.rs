use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use aes::Aes128;
use aes::cipher::{BlockDecrypt, KeyInit};
use aes::cipher::generic_array::GenericArray;
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

pub fn exercise_seven_aes(path: impl AsRef<Path>) {
    // TODO: figure out why it doesn't work
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let joined = lines.fold("".to_string(), |acc, x| {
        format!("{}{}", acc, x.unwrap())
    });

    let bytes = base64_to_bytes(&joined).unwrap();

    let mut blocks = Vec::new();
    let mut iter = bytes.chunks_exact(16);
    while iter.next().is_some() {
        let block: [u8; 16] = iter.next().unwrap().try_into().unwrap();
        blocks.push(GenericArray::from(block));
    }
    let rest = iter.remainder();
    if !rest.is_empty() {
        println!("REMAINDER");
    }

    let key: Vec<u8> = "YELLOW SUBMARINE".bytes().collect();
    let key: [u8; 16] = key.try_into().unwrap();
    let key = GenericArray::from(key);
    let cipher = Aes128::new(&key);

    cipher.decrypt_blocks(&mut blocks);

    let cleartext = blocks.iter().map(|x| x.to_vec()).fold("".to_string(), |acc, x| {
        format!("{}{}", acc, String::from_utf8(x).unwrap())
    });
    println!("{}", cleartext);
}