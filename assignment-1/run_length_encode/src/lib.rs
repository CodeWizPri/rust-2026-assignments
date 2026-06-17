/// Encodes a string slice using Run-Length Encoding (RLE).
/// Example: "AAAAABBBCC" -> [('A', 5), ('B', 3), ('C', 2)]
pub fn run_length_encode(input: &str) -> Vec<(char, u32)> {
    // If the input string is empty, return an empty vector immediately
    if input.is_empty() {
        return Vec::new();
    }

    let mut encoded: Vec<(char, u32)> = Vec::new();
    let mut chars = input.chars();
    
    // Track the first character and start its counter at 1
    let mut current_char = chars.next().unwrap();
    let mut count = 1;

    // Loop through the remaining characters
    for c in chars {
        if c == current_char {
            count += 1;
        } else {
            // Push the completed character tuple to our vector
            encoded.push((current_char, count));
            
            // Reset tracking for the new character group
            current_char = c;
            count = 1;
        }
    }

    // Crucial: Push the very last character group after the loop ends
    encoded.push((current_char, count));

    encoded
}

// Updated unit tests to validate the tuple-vector output format
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_encoding() {
        assert_eq!(
            run_length_encode("AAAAABBBCC"), 
            vec![('A', 5), ('B', 3), ('C', 2)]
        );
    }

    #[test]
    fn test_empty_string() {
        let expected: Vec<(char, u32)> = Vec::new();
        assert_eq!(run_length_encode(""), expected);
    }

    #[test]
    fn test_single_characters() {
        assert_eq!(
            run_length_encode("ABCD"), 
            vec![('A', 1), ('B', 1), ('C', 1), ('D', 1)]
        );
    }
}
