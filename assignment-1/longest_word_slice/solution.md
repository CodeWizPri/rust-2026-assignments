# Solution notes: Longest word slice

## Approach
The algorithm uses zero-cost abstractions in Rust to parse a sentence without allocating any new strings on the heap:

1. **Tokenization:** It utilizes `.split_whitespace()` to iterate through words while automatically filtering out erratic spacing.
2. **Sanitization:** To ensure punctuation doesn't falsely bloat a word's length, `.trim_matches()` strips leading and trailing punctuation (like commas, periods, or exclamation marks).
3. **Tracking Slices:** It compares string slice lengths using `.len()`. Because it takes and holds references (`&str`), the function never copies data; it simply points to the memory addresses of the longest word inside the original sentence input.

### Complexity Analysis
- **Time Complexity:** $O(n)$ where $n$ is the total number of characters in the sentence, scanning the string exactly once.
- **Space Complexity:** $O(1)$ auxiliary space. No new strings are allocated; we only store pointers to parts of the input.

---

## Edge cases handled
- **Punctuation Trimming:** Words like `"internship!"` are evaluated cleanly as `"internship"` so that the punctuation mark doesn't artificially inflate the word length counter.
- **Empty / Blank Sentences:** Strings consisting purely of whitespace return a safe, blank `""` slice right away.
- **Length Ties:** Uses a strict greater-than comparison (`>`) to gracefully return the first matching longest word in the event of a length tie.