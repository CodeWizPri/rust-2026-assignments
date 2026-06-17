# Solution notes: Run-length encode

## Approach
The algorithm processes the input string character by character in a single pass using an iterator (`input.chars()`). 

1. **Initialization:** It tracks the current character group using `current_char` and starts a frequency counter at `1`.
2. **Iteration:** As it steps through the string, it compares the next character to `current_char`:
   - If they match, the consecutive frequency `count` is incremented.
   - If they do not match, it means a distinct group has ended. The pair `(current_char, count)` is pushed into the `encoded` tracking vector. The pointers are then reset to track the new character group.
3. **Finalization:** After the loop finishes, the final accumulated tuple is appended to the vector before returning.

### Complexity Analysis
- **Time Complexity:** $O(n)$ where $n$ is the length of the string, as we iterate through the characters exactly once.
- **Space Complexity:** $O(n)$ in the worst-case scenario where all characters in the string are completely unique (e.g., `"ABCD"` results in a vector of length $n$).

---

## Edge cases handled
- **Empty String (`""`):** Handled immediately at the beginning of the function with an early return of `Vec::new()`, preventing errors when calling `.next().unwrap()` on an empty iterator.
- **Single-Character Inputs:** Correctly outputs a count of `1` for unique sequential items (e.g., `"A"` becomes `[('A', 1)]`).
- **Identical Repeating Run:** Works flawlessly if the entire string consists of only a single repeating character.

---

## Anything special
- **Memory Optimization:** Returning a vector of structured tuples `Vec<(char, u32)>` rather than a flattened string saves execution time and memory allocations, as we avoid converting numerical counts into heap-allocated `String` segments repeatedly during execution.