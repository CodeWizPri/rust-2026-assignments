/// A global compile-time constant array defining the standard lowercase English alphabet.
const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

/// Encrypts an input string slice using a Caesar cipher shift value.
/// Non-alphabetical characters remain unchanged. Case is preserved.
pub fn caesar_cipher(input: &str, shift: i32) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let is_uppercase = c.is_ascii_uppercase();
            let lower_c = c.to_ascii_lowercase();

            // Find the index of the character in our const ALPHABET array
            if let Some(pos) = ALPHABET.iter().position(|&x| x == lower_c) {
                // Handle negative or large shifts correctly using modulo math
                let new_pos = ((pos as i32 + shift) % 26 + 26) % 26;
                let mut shifted_char = ALPHABET[new_pos as usize];

                if is_uppercase {
                    shifted_char = shifted_char.to_ascii_uppercase();
                }
                result.push(shifted_char);
            }
        } else {
            // Numbers, spaces, and punctuation pass through unchanged
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_shift() {
        // Shifting "abc" by 3 yields "def"
        assert_eq!(caesar_cipher("abc", 3), "def");
    }

    #[test]
    fn test_case_preservation() {
        assert_eq!(caesar_cipher("Hello World!", 4), "Lipps Asvph!");
    }

    #[test]
    fn test_negative_shift() {
        // Shifting backward
        assert_eq!(caesar_cipher("def", -3), "abc");
    }

    #[test]
    fn test_large_shift_wrapping() {
        // 26 is a full rotation, so 28 is equivalent to a shift of 2
        assert_eq!(caesar_cipher("xyz", 28), "zab");
    }
}