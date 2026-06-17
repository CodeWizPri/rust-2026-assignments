/// Finds the longest word in a sentence and returns it as a string slice (&str).
/// Words are separated by spaces. If there's a tie, it returns the first one found.
pub fn longest_word_slice(sentence: &str) -> &str {
    // If the string is empty or just spaces, return an empty slice immediately
    if sentence.trim().is_empty() {
        return "";
    }

    let mut longest = "";

    // .split_whitespace() automatically handles multiple spaces, tabs, and newlines
    for word in sentence.split_whitespace() {
        // Clean up punctuation attached to words (like commas or periods)
        let clean_word = word.trim_matches(|c: char| c.is_ascii_punctuation());
        
        if clean_word.len() > longest.len() {
            longest = clean_word;
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_longest() {
        assert_eq!(longest_word_slice("The quick brown fox jumps over the lazy dog"), "quick");
    }

    #[test]
    fn test_empty_and_spaces() {
        assert_eq!(longest_word_slice("   "), "");
        assert_eq!(longest_word_slice(""), "");
    }

    #[test]
    fn test_with_punctuation() {
        assert_eq!(longest_word_slice("Hello world, this is an internship!"), "internship");
    }

    #[test]
    fn test_ties() {
        // "love", "cats" and "dogs" both have length 4, should return the first one ("love")
        assert_eq!(longest_word_slice("I love cats and dogs"), "love");
    }
}