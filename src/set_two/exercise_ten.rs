use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use openssl::symm::{Cipher, decrypt, encrypt};
use crate::set_one::exercise_six::base64_to_bytes;
use crate::set_one::exercise_two::{bytes_to_hex, xor_bytes};
use crate::set_two::exercise_nine::pkcs7_padding;

pub fn pkcs7_unpadding(bytes: &mut Vec<u8>, size: usize) {
    let new_len = bytes.len() - bytes[bytes.len() - 1] as usize;
    bytes.truncate(new_len);
}

pub fn exercise_ten(path: impl AsRef<Path>) {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let joined = lines.fold("".to_string(), |acc, x| {
        format!("{}{}", acc, x.unwrap())
    });

    let mut bytes = base64_to_bytes(&joined).unwrap();
    pkcs7_padding(&mut bytes, 16);
    let key: &[u8] = b"YELLOW SUBMARINE";

    let cleartext = aes128_cbc_decrypt(&bytes, &[0u8; 16], key);
    let cleartext = String::from_utf8(cleartext).unwrap();
    println!("{}", cleartext);
}

fn aes128_encrypt_block(cleartext: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    // TODO: change to Result<>
    if cleartext.len() != 16 || key.len() != 16 {
        return None;
    }
    let mut ciphertext = encrypt(
        Cipher::aes_128_ecb(),
        key,
        None,
        cleartext
    ).unwrap();
    ciphertext.truncate(16);

    Some(ciphertext)
}

fn aes128_decrypt_block(ciphertext: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    // TODO: change to Result<>
    if ciphertext.len() != 16 || key.len() != 16 {
        return None;
    }
    // The OpenSSL call expects a padded cleartext
    let padding = aes128_encrypt_block(&[16u8; 16], key).unwrap();
    let mut data = ciphertext.to_vec();
    data.extend_from_slice(&padding);
    let ciphertext = decrypt(
        Cipher::aes_128_ecb(),
        key,
        None,
        &data
    ).unwrap();

    Some(ciphertext)
}

pub fn aes128_cbc_encrypt(cleartext: &Vec<u8>, iv: &[u8], key: &[u8]) -> Vec<u8> {
    let mut ciphertext: Vec<u8> = Vec::with_capacity(cleartext.len());
    let mut iv = iv.to_vec();
    for block in cleartext.chunks(16) {
        let block = xor_bytes(&iv, &block.to_vec()).unwrap();
        let out = aes128_encrypt_block(&block, &key).unwrap();
        ciphertext.extend_from_slice(&out);
        iv = out;
    }
    ciphertext
}

pub fn aes128_cbc_decrypt(ciphertext: &Vec<u8>, iv: &[u8], key: &[u8]) -> Vec<u8> {
    let mut cleartext: Vec<u8> = Vec::with_capacity(ciphertext.len());
    let mut previous = iv.to_vec();
    for block in ciphertext.chunks(16) {
        let dec = aes128_decrypt_block(&block, &key).unwrap();
        let out = xor_bytes(&previous, &dec.to_vec()).unwrap();
        cleartext.extend_from_slice(&out);
        previous = block.to_vec();
    }

    pkcs7_unpadding(&mut cleartext, 16);
    cleartext
}

#[cfg(test)]
mod tests {
    use crate::set_one::exercise_one::hex_to_bytes;
    use crate::set_one::exercise_two::bytes_to_hex;
    use crate::set_two::exercise_nine::pkcs7_padding;
    use crate::set_two::exercise_ten::{aes128_cbc_decrypt, aes128_cbc_encrypt};

    #[test]
    fn test_aes128_cbc_encrypt() {
        let mut cleartext = "abcdefghijklmnopqrstuvwxyz01234\n".as_bytes().to_vec();
        pkcs7_padding(&mut cleartext, 16);
        assert_eq!(cleartext.len(), 48);
        let iv = "000102030405060708090A0B0C0D0E0F";
        let key = "2b7e151628aed2a6abf7158809cf4f3c";

        let iv = hex_to_bytes(iv).unwrap();
        let key = hex_to_bytes(key).unwrap();

        let ciphertext = aes128_cbc_encrypt(&cleartext, &iv, &key);

        let expected = "940919324e15bbb84c7cf77dbc110a7c978646c48a4c47fd627828f7d997d8a757b87dab22f61de8b6fe4eb6effaed58";

        assert_eq!(expected, bytes_to_hex(&ciphertext));
    }

    #[test]
    fn test_aes128_cbc_decrypt() {
        let expected = "abcdefghijklmnopqrstuvwxyz01234\n";
        let mut cleartext = expected.as_bytes().to_vec();
        pkcs7_padding(&mut cleartext, 16);
        assert_eq!(cleartext.len(), 48);
        let iv = "000102030405060708090A0B0C0D0E0F";
        let key = "2b7e151628aed2a6abf7158809cf4f3c";

        let iv = hex_to_bytes(iv).unwrap();
        let key = hex_to_bytes(key).unwrap();

        let ciphertext = aes128_cbc_encrypt(&cleartext, &iv, &key);

        let result = aes128_cbc_decrypt(&ciphertext, &iv, &key);
        let result = String::from_utf8(result).unwrap();
        assert_eq!(result, expected);
    }
}