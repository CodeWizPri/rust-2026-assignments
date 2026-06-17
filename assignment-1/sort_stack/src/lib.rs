/// Sorts a stack (represented as a Vector) in ascending order using 
/// only an auxiliary temporary stack and standard stack operations (pop, push, peek).
pub fn sort_stack(stack: &mut Vec<i32>) {
    let mut tmp_stack: Vec<i32> = Vec::new();

    // Loop until the main stack is completely empty
    while let Some(current) = stack.pop() {
        // While the temporary stack is not empty AND the top element 
        // of tmp_stack is greater than our current element
        while let Some(&top) = tmp_stack.last() {
            if top < current {
                // Pop from temporary stack and push back onto the main stack
                stack.push(tmp_stack.pop().unwrap());
            } else {
                break;
            }
        }
        
        // Push the current element onto the temporary stack
        tmp_stack.push(current);
    }

    // Move everything back from the temporary stack to the main stack
    // This reverses the order, leaving the main stack perfectly sorted
    while let Some(val) = tmp_stack.pop() {
        stack.push(val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_stack_sorting() {
        let mut stack = vec![34, 3, 31, 98, 92, 23];
        sort_stack(&mut stack);
        assert_eq!(stack, vec![3, 23, 31, 34, 92, 98]);
    }

    #[test]
    fn test_empty_stack() {
        let mut stack: Vec<i32> = Vec::new();
        sort_stack(&mut stack);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_already_sorted() {
        let mut stack = vec![1, 2, 3, 4, 5];
        sort_stack(&mut stack);
        assert_eq!(stack, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut stack = vec![5, 4, 3, 2, 1];
        sort_stack(&mut stack);
        assert_eq!(stack, vec![1, 2, 3, 4, 5]);
    }
}