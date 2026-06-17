use std::collections::HashMap;

/// Groups a list of string slices into clusters of anagrams.
/// Returns a Vector containing Vectors of individual grouped Strings.
pub fn group_anagrams(words: Vec<&str>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for word in words {
        // Create the signature key by sorting the word's characters
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_unstable(); // fast in-place sorting for primitives
        let key: String = chars.into_iter().collect();

        // Use standard Vec::new() here inside or_insert
        groups.entry(key).or_insert_with(Vec::new).push(word.to_string());
    }

    // Extract just the inner values (the clustered string arrays) out of the map
    groups.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_grouping() {
        let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let mut result = group_anagrams(input);

        // Sort outer and inner vectors so asserting equality is deterministic
        for group in &mut result {
            group.sort();
        }
        result.sort_by_key(|g| g.len());

        // Expect 3 independent distinct clusters
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_no_anagrams() {
        let input = vec!["abc", "def", "xyz"];
        let result = group_anagrams(input);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_empty_input() {
        let input: Vec<&str> = Vec::new();
        let result = group_anagrams(input);
        assert!(result.is_empty());
    }
}