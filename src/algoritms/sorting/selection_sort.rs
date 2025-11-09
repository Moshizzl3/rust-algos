fn find_smallest(input_list: &[i32]) -> usize {
    let mut smallest = input_list[0];

    let mut smallest_index: usize = 0;

    for (i, number) in input_list.iter().enumerate() {
        if number < &smallest {
            smallest = *number;
            smallest_index = i;
        }
    }
    smallest_index
}

pub fn selection_sort(input_list: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut cloned_list = input_list.to_vec();

    while !cloned_list.is_empty() {
        let smallest = find_smallest(&cloned_list);

        result.push(cloned_list.swap_remove(smallest))
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_sorted() {
        let input = vec![1, 2, 3, 4, 5];
        let result = selection_sort(&input);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let input = vec![5, 4, 3, 2, 1];
        let result = selection_sort(&input);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_order() {
        let input = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let result = selection_sort(&input);
        assert_eq!(result, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_single_element() {
        let input = vec![42];
        let result = selection_sort(&input);
        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_two_elements() {
        let input = vec![2, 1];
        let result = selection_sort(&input);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_duplicates() {
        let input = vec![3, 3, 1, 2, 3, 1];
        let result = selection_sort(&input);
        assert_eq!(result, vec![1, 1, 2, 3, 3, 3]);
    }

    #[test]
    fn test_all_same() {
        let input = vec![5, 5, 5, 5];
        let result = selection_sort(&input);
        assert_eq!(result, vec![5, 5, 5, 5]);
    }

    #[test]
    fn test_negative_numbers() {
        let input = vec![-3, 1, -5, 2, 0];
        let result = selection_sort(&input);
        assert_eq!(result, vec![-5, -3, 0, 1, 2]);
    }

    #[test]
    fn test_mixed_positive_negative() {
        let input = vec![10, -1, 5, -20, 0, 3];
        let result = selection_sort(&input);
        assert_eq!(result, vec![-20, -1, 0, 3, 5, 10]);
    }

    #[test]
    fn test_large_numbers() {
        let input = vec![1000, 1, 500, 250, 750];
        let result = selection_sort(&input);
        assert_eq!(result, vec![1, 250, 500, 750, 1000]);
    }

    #[test]
    fn test_find_smallest_basic() {
        let input = vec![5, 2, 8, 1, 9];
        assert_eq!(find_smallest(&input), 3); // index of 1
    }

    #[test]
    fn test_find_smallest_first_element() {
        let input = vec![1, 2, 3, 4];
        assert_eq!(find_smallest(&input), 0);
    }

    #[test]
    fn test_find_smallest_last_element() {
        let input = vec![4, 3, 2, 1];
        assert_eq!(find_smallest(&input), 3);
    }

    #[test]
    fn test_find_smallest_duplicates() {
        let input = vec![3, 1, 4, 1, 5];
        assert_eq!(find_smallest(&input), 1); // first occurrence of 1
    }
}
