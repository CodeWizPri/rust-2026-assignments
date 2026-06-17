# Solution notes: Merge two sorted arrays

## Approach
The program merges two pre-sorted arrays efficiently by employing a linear **two-pointer algorithm**, eliminating the need for expensive sorting passes:

1. **Memory Allocation Optimization:** We instantiate the target vector using `Vec::with_capacity(arr1.len() + arr2.len())`. Pre-allocating exact capacity prevents the underlying vector layout from undergoing costly resize re-allocations on the heap during insertion.
2. **Step-by-Step Traversal:** Two tracking indices (`i` and `j`) independently crawl along the inputs. Whichever pointer references the lower current value has its element appended to the merged collection and increments forward.
3. **Tail Cleanup:** Once one array runs completely out of elements, the loop terminates, and any remaining trailing slices from the unfinished array are efficiently appended en masse using `.extend_from_slice()`.

### Complexity Analysis
- **Time Complexity:** $O(n + m)$ where $n$ and $m$ represent the respective lengths of the input arrays. Every element is visited and moved exactly once.
- **Space Complexity:** $O(n + m)$ to store the combined elements in the returned vector.

---

## Edge cases handled
- **Asymmetric Vector Lengths:** The algorithm transitions smoothly into tail cleanups when processing inputs of mismatched sizing footprints.
- **Empty Constraints:** If a parameter is passed empty (`[]`), the loop bypasses comparison entirely and appends the non-empty list safely.
- **Identical Overlapping Values:** Duplicate numbers are safely processed without getting dropped or corrupting order sequences.