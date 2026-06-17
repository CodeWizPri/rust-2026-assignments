# Solution notes: Caesar cipher with a const alphabet

## Approach
The algorithm implements a bounded cipher shift mapping over a predefined compile-time character stack:

1. **Static Reference Lookup:** A fixed `const ALPHABET` array tracks standard lowercase characters. Using `const` avoids runtime heap allocations or repeated re-allocations for the reference mapping bounds.
2. **Character Transformation:** As we traverse the string elements, casing properties are isolated via `.is_ascii_uppercase()`, and characters are indexed against the target array using `.position()`.
3. **Mathematical Wrapping:** To handle arbitrary shifts (including negative shifts or numbers exceeding the alphabet bounds), we implement a double modulo normalization formula: `((pos + shift) % 26 + 26) % 26`. This guarantees an index fallback within the safe array limit range of `0..25`.

### Complexity Analysis
- **Time Complexity:** $O(n)$ where $n$ is the total length of the text block. Looking up an item in a fixed-size array of 26 characters executes in $O(1)$ constant steps.
- **Space Complexity:** $O(n)$ to house the transformed output string sequence.

---

## Edge cases handled
- **Negative Shifting Parameters:** Handles negative shifts elegantly without triggering underflow array panics due to the wrapping math architecture.
- **Symbol / Numeric Protection:** Non-alphabetic properties (like spaces or punctuation marks) pass through directly to avoid payload distortion.
- **Large Shift Wrapping:** Shifts greater than 26 wrap around smoothly via structural remainder cycles.