use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref FREQ: HashMap<char, f64> = {
        const FREQ_DATA: &str = include_str!("freq.csv");
        let lines = FREQ_DATA.lines();

        let mut frequencies: HashMap<char, f64> = HashMap::new();
        lines.for_each(|x| {
            let line: Vec<&str> = x.split(',').collect();
            assert_eq!(2, line.len());
            let k: char = unsafe { char::from(*line[0].as_ptr()) };
            frequencies.insert(k, line[1].parse().unwrap());
        });

        frequencies
    };
}

pub fn str_to_base64(src: &str) -> String {
    to_base64(src.as_bytes())
}

pub fn to_base64(src: &[u8]) -> String {
    const PAD: char = '=';
    const MASK: u32 = 0x3F;
    static BASE64_CHARS: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    let mut result = String::new();

    let chunks = src.chunks_exact(3);
    for chunk in chunks {
        let combined = (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | chunk[2] as u32;
        result.push(BASE64_CHARS[(combined >> 18 & MASK) as usize]);
        result.push(BASE64_CHARS[(combined >> 12 & MASK) as usize]);
        result.push(BASE64_CHARS[(combined >> 6 & MASK) as usize]);
        result.push(BASE64_CHARS[(combined & MASK) as usize]);
    }

    /*    let remainder = chunks.remainder();
    let rem = remainder.len();
    if rem > 0 {
        // rem can be only 1 or 2 since otherwise it would be a full chunk, handled above

        if rem == 1 {
            result.push(BASE64_CHARS[((remainder[0] >> 6) as u32 & MASK) as usize]);
            result.push(PAD);
            result.push(PAD);
        } else {
            let combined = (remainder[0] as u32) << 8 | remainder[1] as u32;
            result.push(BASE64_CHARS[(combined >> 6 & MASK) as usize]);
            result.push(BASE64_CHARS[(combined & MASK) as usize]);
            result.push(PAD);
        }
    }*/

    result
}

pub fn frequency(input: &str) -> HashMap<char, f64> {
    frequency_bytes(input.as_bytes())
}

pub fn frequency_bytes(input: &[u8]) -> HashMap<char, f64> {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut calculated_frequencies: HashMap<char, f64> = HashMap::new();

    ALPHABET.chars().for_each(|c| {
        for b in input {
            if (c as u8 ^ *b) == 0 {
                let freq = calculated_frequencies.entry(c).or_insert(0.0);
                let normalized_key = c.to_ascii_lowercase();
                *freq += FREQ.get(&normalized_key).unwrap();
            }
        }
    });

    calculated_frequencies
}

pub fn xor(x: &str, y: &str) -> String {
    xor_bytes(hex_string_to_bytes(x), hex_string_to_bytes(y))
}

pub fn xor_bytes(x: Vec<u8>, y: Vec<u8>) -> String {
    assert_eq!(0, x.len() % 2);
    assert_eq!(0, y.len() % 2);
    assert!(x.len() >= y.len());

    let mut result = String::new();

    for (idx, x_val) in x.iter().enumerate() {
        let xor_val = x_val ^ y.get(idx % y.len()).unwrap();
        let (hi, low) = byte_to_hex_chars(xor_val);
        result.push(hi);
        result.push(low);
    }

    result
}

fn hex_string_to_bytes(hex: &str) -> Vec<u8> {
    assert_eq!(hex.len() % 2, 0);
    assert!(hex.is_ascii());

    let mut result = Vec::with_capacity(hex.len() / 2);

    let chars = hex.as_bytes().chunks_exact(2);
    for chunk in chars {
        let hi = hex_char_to_hex_value(chunk[0] as char);
        let low = hex_char_to_hex_value(chunk[1] as char);
        let byte = hi << 4 | low;
        result.push(byte);
    }

    result
}

fn hex_char_to_hex_value(hex: char) -> u8 {
    assert!(char::is_ascii_hexdigit(&hex));
    match hex {
        '0'..='9' => (hex as u8) - b'0',
        'A'..='F' => (hex as u8) - b'A' + 10,
        'a'..='f' => (hex as u8) - b'a' + 10,
        _ => panic!(),
    }
}

fn byte_to_hex_chars(b: u8) -> (char, char) {
    let hi = (b & 0xF0) >> 4;
    let low = b & 0x0F;
    (nib_to_char(hi), nib_to_char(low))
}

fn bytes_to_hex_string(bytes: Vec<u8>) -> String {
    let mut result = String::new();
    for b in bytes {
        let (hi, low) = byte_to_hex_chars(b);
        result.push(hi);
        result.push(low)
    }
    dbg!(result)
}

fn nib_to_char(n: u8) -> char {
    match n {
        0..=9 => (n + b'0') as char,
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_char_byte() {
        assert_eq!(15, hex_char_to_hex_value('F'));
        assert_eq!(15, hex_char_to_hex_value('f'));
        assert_eq!(1, hex_char_to_hex_value('1'));
    }

    #[test]
    fn test_hex_string_to_bytes() {
        let input = "6c61d277320657965e";
        let expected = vec![0x6c, 0x61, 0xd2, 0x77, 0x32, 0x06, 0x57, 0x96, 0x5e];
        assert_eq!(expected, *hex_string_to_bytes(input));
    }

    #[test]
    fn test_frequencies() {
        let input = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let frequencies = frequency(input);
        frequencies.iter().for_each(|(k, v)| {
            assert_eq!(FREQ.get(&k.to_ascii_lowercase()).unwrap(), v);
        });
    }

    /// https://www.cryptopals.com/sets/1/challenges/1
    #[test]
    fn test_base64_encoding() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(expected, to_base64(hex_string_to_bytes(input).as_slice()));
    }

    #[test]
    fn test_base64_encoding2() {
        let input = "Many hands make light work.";
        let expected = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu";

        assert_eq!(expected, str_to_base64(input));
    }

    /// https://www.cryptopals.com/sets/1/challenges/2
    #[test]
    fn test_xor() {
        let i1 = "1c0111001f010100061a024b53535009181c";
        let i2 = "686974207468652062756c6c277320657965";
        let expected = "746865206B696420646F6E277420706C6179";

        assert_eq!(expected, xor(i1, i2));
    }

    /// https://www.cryptopals.com/sets/1/challenges/3
    #[test]
    fn xor_cipher_decrypt() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let bytes = hex_string_to_bytes(input);
        if let Some(highest_frequency_key) = frequency_bytes(&bytes[..])
            .into_iter()
            .max_by(|(_, x), (_, x1)| x.total_cmp(x1))
            .map(|(k, _)| String::from(k))
        {
            assert_eq!("x", highest_frequency_key);

            let hex_string = bytes_to_hex_string(
                highest_frequency_key
                    .to_ascii_uppercase()
                    .as_bytes()
                    .to_vec(),
            );
            let result = xor_bytes(bytes, hex_string.as_bytes().to_vec());
            assert_eq!("Cooking MC's like a pound of bacon", result);
        }
    }
}
