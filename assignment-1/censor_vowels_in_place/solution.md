# Solution notes: Censor vowels in place

## Approach
This exercise requires mutating data in-place via a mutable reference (`&mut str`), adhering strictly to Rust's aliasing and exclusive mutability guarantees. 

1. **Byte Mutation:** Because ASCII characters map 1:1 to single bytes in UTF-8 encoding, we can modify the characters safely using `.as_bytes_mut()`. This allows us to circumvent structural re-allocations.
2. **Pattern Matching:** We iterate through the raw byte references and use a `match` control flow block to isolate the byte expressions corresponding to uppercase and lowercase standard ASCII vowels (`A`, `E`, `I`, `O`, `U`).
3. **Dereference Assignment:** When a vowel byte is matched, we dereference the pointer using `*byte` and overwrite the data directly with the asterisk byte value (`b'*'`).

### Complexity Analysis
- **Time Complexity:** $O(n)$ where $n$ is the length of the string in bytes. The algorithm performs a single, sequential pass over the data.
- **Space Complexity:** $O(1)$ auxiliary space. Mutation happens directly inside the pre-allocated memory buffer of the caller; zero heap allocations occur.

---

## Edge cases handled
- **No Vowels Present:** Strings composed purely of consonants, spaces, or numbers complete execution unmodified without throwing errors.
- **Mixed Casing:** The matching sequence treats `e` and `E` uniformly, ensuring robust case-insensitive filtering.
- **Empty Constraints:** Blank allocations (`""`) evaluate gracefully without index-out-of-bounds panics since the byte loop bounds automatically evaluate to zero.