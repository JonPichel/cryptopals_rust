
pub fn pkcs7_padding(bytes: &mut Vec<u8>, size: usize) {
    if size < 2 {
        return;
    }

    let padding = size - (bytes.len() % size);
    for _ in 0..padding {
        bytes.push(padding as u8);
    }
}

#[cfg(test)]
mod tests {
    use crate::set_two::exercise_nine::pkcs7_padding;

    #[test]
    fn test_pkcs7_padding() {
        let mut bytes: Vec<u8> = "YELLOW_SUBMARINE".bytes().collect();
        let expected: Vec<u8> = "YELLOW_SUBMARINE\x04\x04\x04\x04".bytes().collect();

        pkcs7_padding(&mut bytes, 20);
        assert_eq!(&bytes, &expected);
    }
}