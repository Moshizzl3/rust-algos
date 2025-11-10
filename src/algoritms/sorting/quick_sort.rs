use std::cmp::Ordering;

pub fn quick_sort<T: Clone + Copy + Ord>(input_list: &[T]) -> Vec<T> {
    if input_list.len() < 2 {
        return input_list.to_vec();
    }

    let (mut less, mut greater, mut equal) = (Vec::new(), Vec::new(), Vec::new());

    let pivot = input_list[input_list.len() / 2];

    input_list
        .iter()
        .copied()
        .for_each(|x| match x.cmp(&pivot) {
            Ordering::Less => less.push(x),
            Ordering::Greater => greater.push(x),
            Ordering::Equal => equal.push(x),
        });

    quick_sort(&less)
        .into_iter()
        .chain(equal)
        .chain(quick_sort(&greater))
        .collect()
}

mod test {
    use super::*;

    #[test]
    fn test_quicksort_positive_numbers() {
        let list = vec![2, 4, 1, 3, 5];
        let result = quick_sort(&list);
        assert_eq!(result, [1, 2, 3, 4, 5])
    }

    #[test]
    fn test_quicksort_negative_numbers() {
        let list = vec![-2, -4, -1, -3, -5];
        let result = quick_sort(&list);
        assert_eq!(result, [-5, -4, -3, -2, -1])
    }

    #[test]
    fn test_quicksort_mixed_numbers() {
        let list = vec![4, 1, -2, -4, 3, -1, 2, -3, -5, 5, 0];
        let result = quick_sort(&list);
        assert_eq!(result, [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5])
    }

    #[test]
    fn test_quicksort_with_duplicates() {
        let list = vec![3, 1, 3, 2, 3];
        let result = quick_sort(&list);
        assert_eq!(result, [1, 2, 3, 3, 3]);
    }

    #[test]
    fn test_quicksort_all_same() {
        let list = vec![5, 5, 5, 5];
        let result = quick_sort(&list);
        assert_eq!(result, [5, 5, 5, 5]);
    }

    #[test]
    fn test_quicksort_empty() {
        let list: Vec<i32> = vec![];
        let result = quick_sort(&list);
        assert_eq!(result, []);
    }

    #[test]
    fn test_quicksort_already_sorted() {
        let list = vec![1, 2, 3, 4, 5];
        let result = quick_sort(&list);
        assert_eq!(result, [1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_quicksort_single_element() {
        let list = vec![42];
        let result = quick_sort(&list);
        assert_eq!(result, [42]);
    }

    #[test]
    fn test_quicksort_two_elements_sorted() {
        let list = vec![1, 2];
        let result = quick_sort(&list);
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn test_quicksort_two_elements_unsorted() {
        let list = vec![2, 1];
        let result = quick_sort(&list);
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn test_quicksort_reverse_sorted() {
        let list = vec![5, 4, 3, 2, 1];
        let result = quick_sort(&list);
        assert_eq!(result, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quicksort_many_duplicates() {
        let list = vec![1, 5, 3, 5, 2, 5, 4, 5, 5];
        let result = quick_sort(&list);
        assert_eq!(result, [1, 2, 3, 4, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_quicksort_with_zeros() {
        let list = vec![0, -1, 0, 1, 0];
        let result = quick_sort(&list);
        assert_eq!(result, [-1, 0, 0, 0, 1]);
    }

    #[test]
    fn test_quicksort_large_range() {
        let list = vec![100, -100, 50, -50, 0];
        let result = quick_sort(&list);
        assert_eq!(result, [-100, -50, 0, 50, 100]);
    }

    // testing that generics work with other types
    #[test]
    fn test_quicksort_strings() {
        let list = vec!["dog", "cat", "zebra", "ant", "bear"];
        let result = quick_sort(&list);
        assert_eq!(result, ["ant", "bear", "cat", "dog", "zebra"]);
    }

    #[test]
    fn test_quicksort_chars() {
        let list = vec!['z', 'a', 'm', 'b', 'y'];
        let result = quick_sort(&list);
        assert_eq!(result, ['a', 'b', 'm', 'y', 'z']);
    }

    #[test]
    fn test_quicksort_larger_list() {
        let list = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];
        let result = quick_sort(&list);
        assert_eq!(result, [2, 3, 5, 6, 7, 9, 10, 11, 12, 14]);
    }
}
