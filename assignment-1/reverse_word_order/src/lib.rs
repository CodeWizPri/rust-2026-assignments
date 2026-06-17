/// Takes a string slice and returns a new String with the words in reverse order.
/// Spaces are preserved as single space delimiters between words.
pub fn reverse_word_order(sentence: &str) -> String {
    // Split the sentence into words, reverse the iterator sequence, 
    // and collect them into a Vector of string slices
    let words: Vec<&str> = sentence.split_whitespace().rev().collect();

    // Join the reversed slices back together with a single space in between
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_reverse() {
        assert_eq!(
            reverse_word_order("The quick brown fox"), 
            "fox brown quick The"
        );
    }

    #[test]
    fn test_single_word() {
        assert_eq!(reverse_word_order("Rust"), "Rust");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(reverse_word_order(""), "");
        assert_eq!(reverse_word_order("   "), "");
    }

    #[test]
    fn test_multiple_spaces() {
        // Extra inner or outer padding spaces should clean up to single spaces
        assert_eq!(
            reverse_word_order("  hello   world  "), 
            "world hello"
        );
    }
}