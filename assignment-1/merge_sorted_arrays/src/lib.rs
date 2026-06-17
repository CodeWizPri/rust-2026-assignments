/// Merges two pre-sorted integer slices into a single, fully sorted Vector.
/// Uses the two-pointer technique for optimal linear time execution.
pub fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    // Loop while there are still elements to compare in both arrays
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    // If there are leftover elements in arr1, append them all
    if i < arr1.len() {
        merged.extend_from_slice(&arr1[i..]);
    }

    // If there are leftover elements in arr2, append them all
    if j < arr2.len() {
        merged.extend_from_slice(&arr2[j..]);
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_merge() {
        let a = [1, 3, 5, 7];
        let b = [2, 4, 6, 8];
        assert_eq!(merge_sorted_arrays(&a, &b), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_different_lengths() {
        let a = [1, 2];
        let b = [3, 4, 5, 6];
        assert_eq!(merge_sorted_arrays(&a, &b), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_one_empty_array() {
        let a = [];
        let b = [1, 2, 3];
        assert_eq!(merge_sorted_arrays(&a, &b), vec![1, 2, 3]);
    }

    #[test]
    fn test_duplicates() {
        let a = [1, 2, 2, 4];
        let b = [2, 3, 4, 5];
        assert_eq!(merge_sorted_arrays(&a, &b), vec![1, 2, 2, 2, 3, 4, 4, 5]);
    }
}