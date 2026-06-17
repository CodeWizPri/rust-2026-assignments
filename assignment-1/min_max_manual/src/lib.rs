/// Finds the minimum and maximum values in an integer slice manually.
/// Returns a tuple containing Option values: `(Option<i32>, Option<i32>)`.
/// If the slice is empty, returns `(None, None)`.
pub fn min_max_without_helpers(nums: &[i32]) -> (Option<i32>, Option<i32>) {
    // If the slice is empty, return None immediately
    if nums.is_empty() {
        return (None, None);
    }

    // Initialize min and max with the first element of the slice
    let mut min_val = nums[0];
    let mut max_val = nums[0];

    // Manually loop through the rest of the slice elements
    for &num in nums.iter().skip(1) {
        if num < min_val {
            min_val = num;
        }
        if num > max_val {
            max_val = num;
        }
    }

    // Wrap the found values in Some()
    (Some(min_val), Some(max_val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_array() {
        assert_eq!(min_max_without_helpers(&[3, 1, 4, 1, 5, 9, -2, 6]), (Some(-2), Some(9)));
    }

    #[test]
    fn test_single_element() {
        assert_eq!(min_max_without_helpers(&[42]), (Some(42), Some(42)));
    }

    #[test]
    fn test_empty_array() {
        assert_eq!(min_max_without_helpers(&[]), (None, None));
    }

    #[test]
    fn test_all_identical() {
        assert_eq!(min_max_without_helpers(&[7, 7, 7, 7]), (Some(7), Some(7)));
    }
}
