# Solution notes: min_max without iterator helpers

## Approach
The algorithm finds both the minimum and maximum values in a single sequential scan without relying on built-in iterator methods like `.min()` or `.max()`.

1. **Safety Verification:** It checks if the slice collection is empty up front. If empty, it returns `(None, None)` to safely mirror Rust's core pointer idioms.
2. **State Seeding:** It assumes the first element (`nums[0]`) is concurrently both the minimum and maximum tracking boundary baseline.
3. **Linear Scan:** It evaluates the remaining slice elements one by one. By matching values against the active extreme conditions using traditional conditional evaluation checks (`if`), it replaces the values in-place dynamically.

### Complexity Analysis
- **Time Complexity:** $O(n)$ where $n$ is the total count of numbers in the array slice. It iterates through the collection elements exactly once.
- **Space Complexity:** $O(1)$ constant auxiliary memory footprints, as tracking values are simple scalar variables bound to stack registers.

---

## Edge cases handled
- **Empty Slices:** Prevents index out of bounds runtime exceptions via early matching and returning `None` pairs.
- **Single-Element Arrays:** Correctly concludes that the lone integer holds both the maximum and minimum designation concurrently.
- **Negative Integer Tracking:** Correctly tracks negative scales since bounds are evaluated via strict primitive integer comparison boundaries.