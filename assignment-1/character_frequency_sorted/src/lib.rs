use std::collections::HashMap;

/// Counts the frequency of each character in the input string and returns
/// a vector of tuples `(char, u32)` sorted in descending order by frequency.
/// If frequencies are equal, characters are sorted alphabetically.
pub fn character_frequency_sorted(input: &str) -> Vec<(char, u32)> {
    let mut frequencies: HashMap<char, u32> = HashMap::new();

    // Step 1: Count occurrences of each character
    for c in input.chars() {
        // .entry() checks if the character exists; if not, it inserts 0, then increments by 1
        *frequencies.entry(c).or_insert(0) += 1;
    }

    // Step 2: Collect the HashMap pairs into a mutable Vector so we can sort them
    let mut result: Vec<(char, u32)> = frequencies.into_iter().collect();

    // Step 3: Sort the vector
    // sort_by() lets us define custom sorting rules:
    // We want descending order for frequency (b.1.cmp(&a.1)), 
    // and ascending alphabetical order as a tie-breaker (a.0.cmp(&b.0)).
    result.sort_by(|a, b| {
        match b.1.cmp(&a.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0), // Tie-breaker: alphabetical
            other => other,                            // Otherwise, highest frequency wins
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_frequencies() {
        // 'a' appears 3 times, 'b' appears 2 times, 'c' appears 1 time
        assert_eq!(
            character_frequency_sorted("bbaaac"),
            vec![('a', 3), ('b', 2), ('c', 1)]
        );
    }

    #[test]
    fn test_sorting_ties() {
        // 'a', 'b', and 'c' all appear 2 times. They should be sorted alphabetically.
        assert_eq!(
            character_frequency_sorted("bbccaa"),
            vec![('a', 2), ('b', 2), ('c', 2)]
        );
    }

    #[test]
    fn test_empty_string() {
        let expected: Vec<(char, u32)> = Vec::new();
        assert_eq!(character_frequency_sorted(""), expected);
    }
}