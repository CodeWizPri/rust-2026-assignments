# Solution notes: Character frequency, sorted

## Approach
The problem is solved efficiently using a hash map combined with a custom vector sorting rule:

1. **Frequency Mapping:** We iterate through the string's characters and use a `HashMap<char, u32>`. By utilizing the elegant `.entry().or_insert()` API pattern in Rust, we can update or create frequencies in a single line safely.
2. **Vector Extraction:** Since hash maps are naturally unordered collections, we consume the map via `.into_iter().collect()` to move the element pairs into a sliceable `Vec<(char, u32)>`.
3. **Dual-Criteria Sorting:** We use `.sort_by()` to specify sorting logic. We prioritize the integer frequency count in descending order. If the comparison yields `Ordering::Equal`, our custom match arm branches into a secondary fallback comparison that sorts characters in ascending alphabetical order.

### Complexity Analysis
- **Time Complexity:** $O(n + k \log k)$ where $n$ is the total length of the text sequence (to build the map), and $k$ is the count of distinct unique characters (to sort the vector elements).
- **Space Complexity:** $O(k)$ auxiliary memory allocations required to host unique entries inside our local map tracking frame.

---

## Edge cases handled
- **Frequency Ties:** Characters with identical counts do not appear randomly; they are perfectly stabilized alphabetically through our custom comparator logic.
- **Empty Constraints:** Blanks return an empty vector allocation instantly without any sorting actions triggering.
- **Special Characters:** Spaces, symbols, and punctuation are naturally isolated and tracked as valid unique key properties without exception.