
fn repeating_key_xor(bytes: &mut [u8], key: &[u8]) {
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte ^= &key[(index % key.len()) as usize];
    }
}

#[cfg(test)]
mod tests {
    use crate::set_one::exercise_two::bytes_to_hex;
    use super::*;

    #[test]
    fn test_repeating_key_xor() {
        let cleartext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let mut bytes = cleartext.as_bytes().to_vec();
        repeating_key_xor(&mut bytes, key.as_bytes());
        assert_eq!(bytes_to_hex(&bytes), expected);

        repeating_key_xor(&mut bytes, key.as_bytes());
        assert_eq!(String::from_utf8(bytes).unwrap(), cleartext);
    }
}