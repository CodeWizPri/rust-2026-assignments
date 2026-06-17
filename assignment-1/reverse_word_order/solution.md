# Solution notes: Reverse the word order

## Approach
The solution leverages Rust's powerful iterator adapters to modify word layout cleanly:

1. **Splitting:** `.split_whitespace()` breaks the sentence down by tokenizing text around spaces, eliminating any irregular or redundant spaces.
2. **Reversing:** The `.rev()` modifier flips the sequence direction of the underlying string slices without copying any actual word data.
3. **Recombining:** The reversed items are collected into a vector (`Vec<&str>`), and `.join(" ")` stitches them together into a final heap-allocated `String` with uniform single-space gaps.

### Complexity Analysis
- **Time Complexity:** $O(n)$ where $n$ is the number of characters in the input sentence. We read through the sentence to split it, reverse the references, and allocate the new string layout.
- **Space Complexity:** $O(n)$ because we build and return an entirely new `String` containing the reversed sequence.

---

## Edge cases handled
- **Excessive Spacing:** Multiple adjacent internal spaces or trailing padding elements are automatically trimmed away into a single canonical space layout.
- **Blank Spaces / Empty Strings:** Return a completely empty `""` string dynamically without allocations blowing up or throwing runtime panics.
- **Single Word Inputs:** Correctly leaves the word unchanged without attaching unwanted leading or trailing white spaces.