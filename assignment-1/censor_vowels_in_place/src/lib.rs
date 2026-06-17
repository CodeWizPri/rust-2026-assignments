/// Censors all vowels (A, E, I, O, U - case insensitive) in a mutable string slice 
/// by replacing them with an asterisk ('*') in-place.
pub fn censor_vowels_in_place(string: &mut str) {
    // We access the underlying raw bytes mutably to change them directly in memory
    let bytes = unsafe { string.as_bytes_mut() };

    for byte in bytes {
        // Match both lowercase and uppercase ASCII vowels
        match *byte {
            b'a' | b'e' | b'i' | b'o' | b'u' |
            b'A' | b'E' | b'I' | b'O' | b'U' => {
                *byte = b'*'; // Replace the byte value with an asterisk
            }
            _ => {} // Do nothing for consonants, spaces, and punctuation
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_censoring() {
        let mut text = String::from("Hello World");
        censor_vowels_in_place(&mut text);
        assert_eq!(text, "H*ll* W*rld");
    }

    #[test]
    fn test_all_vowels() {
        let mut text = String::from("AeIoU");
        censor_vowels_in_place(&mut text);
        assert_eq!(text, "*****");
    }

    #[test]
    fn test_no_vowels() {
        let mut text = String::from("Rhythms fly.");
        censor_vowels_in_place(&mut text);
        assert_eq!(text, "Rhythms fly.");
    }

    #[test]
    fn test_empty_string() {
        let mut text = String::from("");
        censor_vowels_in_place(&mut text);
        assert_eq!(text, "");
    }
}