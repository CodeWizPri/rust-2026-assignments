# Solution notes: Group anagrams together

## Approach
The algorithm groups collections of structural permutations efficiently via a map index design pattern:

1. **Signature Standardization:** We loop through the word slice elements and extract their individual characters. By processing the sequence through `.sort_unstable()`, we sort the characters alphabetically to construct an immutable uniform lookup string (e.g., `"eat"`, `"tea"`, and `"ate"` all yield the signature `"aet"`).
2. **Dynamic Entry Grouping:** We utilize a `HashMap<String, Vec<String>>`. The sorted signature acts as the key map target. Using the `.entry().or_insert()` layout, we cleanly append the original owned word string translation into its respective similarity bucket.
3. **Value Aggregation:** Once all inputs are processed, the function consumes the hash map properties via `.into_values().collect()` to export only the nested arrays.

### Complexity Analysis
- **Time Complexity:** $O(n \cdot m \log m)$ where $n$ is the total number of words in the vector, and $m$ is the maximum length of an individual string token (due to the character sorting operation).
- **Space Complexity:** $O(n \cdot m)$ required to hold the structural elements within the dictionary and output buffers.

---

## Edge cases handled
- **Isolated Elements:** Independent strings that possess unique sequences form separate individual vector arrays cleanly.
- **Empty Constraints:** Blank primary argument states return an unallocated vector response frame instantly.
- **Identical Entries:** Duplicate occurrences of identical items group into the same bucket safely without collisions.