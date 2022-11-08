use crate::set_one::error::SetOneError;
use crate::set_one::exercise_one::hex_to_bytes;

pub fn xor_hex(a: &str, b: &str) -> Result<String, SetOneError> {
    let a = hex_to_bytes(a)?;
    let b = hex_to_bytes(b)?;
    let bytes = xor_bytes(&a, &b)?;
    Ok(bytes_to_hex(&bytes[..]))
}

pub fn xor_bytes(a: &Vec<u8>, b: &Vec<u8>) -> Result<Vec<u8>, SetOneError> {
    if a.len() != b.len() {
        return Err(SetOneError::IncompatibleSize(a.len(), b.len()));
    }
    let mut c: Vec<u8> = Vec::new();
    for (a, b) in a.iter().zip(b.iter()) {
        c.push(a ^ b);
    }
    Ok(c)
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::new();
    for byte in bytes {
        let h: u8 = byte >> 4;
        let l: u8 = byte & 0xf;
        for value in [h, l] {
            match value {
                0..=9 => {
                    hex.push((b'0' + value) as char);
                }
                10..=15 => {
                    hex.push((b'a' + value - 10) as char);
                }
                _ => {
                    panic!("Error extracting values!");
                }
            }
        }
    }
    hex
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xor_hex() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        assert_eq!(xor_hex(a, b).unwrap(), "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn test_xor_bytes() {
        let a: Vec<u8> = vec![0b10101010, 0b01010101, 0b00000000, 0b11111111];
        let b: Vec<u8> = vec![0b11111111, 0b11111111, 0b11111111, 0b11111111];
        assert_eq!(xor_bytes(&a, &b).unwrap(), vec![0b01010101u8, 0b10101010u8, 0b11111111u8, 0b00000000u8]);
    }

    #[test]
    fn test_bytes_to_hex() {
        let bytes: [u8; 4] = [0x35, 0xff, 0x20, 0x4c];
        assert_eq!(bytes_to_hex(&bytes), "35ff204c");
    }
}