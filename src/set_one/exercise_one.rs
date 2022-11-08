use crate::set_one::error::SetOneError;


pub fn hex_to_base64(hex: &str) -> Result<String, SetOneError> {
    Ok(bytes_to_base64(hex_to_bytes(hex)?.as_ref()))
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, SetOneError> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut byte = 0u8;
    for (index, char) in hex.bytes().enumerate() {
        match char {
            b'0'..=b'9' => {
                byte = (byte << 4) + (char - b'0');
            }
            b'a'..=b'f' => {
                byte = (byte << 4) + (char - b'a' + 10);
            }
            b'A'..=b'F' => {
                byte = (byte << 4) + (char - b'A' + 10);
            }
            _ => return Err(SetOneError::InvalidHex(hex.to_string())),
        }
        if index % 2 == 1 {
            bytes.push(byte);
            byte = 0;
        }
    }
    Ok(bytes)
}

pub fn bytes_to_base64(bytes: &[u8]) -> String {
    let mut values: Vec<u8> = Vec::new();

    let mut value: u8 = 0;
    let mut bits_left_byte: u8;
    let mut bits_left_value: u8 = 6;
    let mut padding: u8 = 3;
    for byte in bytes {
        bits_left_byte = 8;
        while bits_left_byte > 0 {
            if bits_left_value == 0 {
                bits_left_value = 6;
            }
            if bits_left_value <= bits_left_byte {
                // Will complete the value
                value |= byte >> (bits_left_byte - bits_left_value) & 0b111111;
                bits_left_byte -= bits_left_value;
                bits_left_value = 0;
                values.push(value);
                value = 0;
            } else {
                // Won't complete the value
                value = (byte << (8 - bits_left_byte)) >> 2;
                bits_left_value -= bits_left_byte;
                bits_left_byte = 0;
            }
        }
        if padding > 0 {
            padding -= 1;
        } else {
            padding = 2;
        }
    }

    if bits_left_value != 0 {
        values.push(value);
    }

    let mut base64 = String::new();
    for value in &values {
        // A..=Z -> 0..=25
        // a..=z -> 26..=51
        // 0..=9 -> 52..=61
        // + -> 62, / -> 63
        match value {
            0..=25 => {
                base64.push((b'A' + value) as char);
            }
            26..=51 => {
                base64.push((b'a' + value - 26) as char);
            }
            52..=61 => {
                base64.push((b'0' + value - 52) as char);
            }
            62 => {
                base64.push('+');
            }
            63 => {
                base64.push('/');
            }
            _ => {
                panic!("Error extracting values!");
            }
        }
    }
    base64.push_str("=".repeat(padding as usize).as_ref());
    base64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(hex_to_base64(hex).unwrap(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn test_hex_to_bytes_valid() {
        let hex = "012b3c4f";
        assert_eq!(hex_to_bytes(hex).unwrap(), [1, 43, 60, 79]);
    }

    #[test]
    fn test_hex_to_bytes_invalid_ascii() {
        let hex = "invalid_chars";
        assert!(hex_to_bytes(hex).is_err());
    }

    #[test]
    fn test_hex_to_bytes_invalid_unicode() {
        let hex = "áéíóú";
        assert!(hex_to_bytes(hex).is_err());
    }

    #[test]
    fn test_bytes_to_base64_no_padding() {
        let bytes = [20, 40, 60];
        assert_eq!(bytes_to_base64(&bytes), "FCg8");
    }

    #[test]
    fn test_bytes_to_base64_padding() {
        let bytes = [20, 40, 60, 80];
        assert_eq!(bytes_to_base64(&bytes), "FCg8UA==");
    }

    #[test]
    fn test_bytes_to_base64_multiple_quantum() {
        let bytes = [20, 40, 60, 80, 100, 120, 140];
        assert_eq!(bytes_to_base64(&bytes), "FCg8UGR4jA==");
    }
}