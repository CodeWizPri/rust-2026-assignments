# Solution notes: Sort a stack using a temporary stack

## Approach
The algorithm sorts a stack mutably in-place by utilizing an auxiliary temporary stack while strictly adhering to LIFO (Last In, First Out) constraints:

1. **Popping Elements:** The main input stack is parsed iteratively by popping elements one by one from the end of the vector (`stack.pop()`).
2. **Value Evaluation & Monotonicity:** Before pushing the popped value onto the `tmp_stack`, we inspect its top element using `.last()`. If the top item of `tmp_stack` is smaller than our active element, we pop it off `tmp_stack` and push it back onto our primary `stack`. This structural tumbling ensures that elements inside `tmp_stack` are maintained in a strictly ascending order layout.
3. **Restoration:** Once the source stack elements are exhausted, `tmp_stack` holds the values sorted with the largest values at the top. Popping elements back into the primary stack naturally flips the sequence so that the smallest elements end up at the top (the end of the vector), perfectly matching the ascending sorting criteria of the test suite.

### Complexity Analysis
- **Time Complexity:** $O(n^2)$ where $n$ is the total number of items on the stack. In the worst-case scenario (e.g., an already perfectly sorted array), we have to cycle elements back and forth between the stacks repeatedly.
- **Space Complexity:** $O(n)$ auxiliary space to host the shifted elements inside our temporary stack vector.

---

## Edge cases handled
- **Pre-Sorted Stacks:** Handled gracefully by matching the active inversion constraints and flipping the vector sequence correctly.
- **Empty Stacks:** The outer matching conditional `while let` pattern skips execution entirely, preventing stack underflow panics.
- **Duplicates:** Values equal to the current tracking boundary remain tightly grouped inside the shifting cycle seamlessly without getting lost.