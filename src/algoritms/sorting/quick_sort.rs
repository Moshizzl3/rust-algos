use std::cmp::Ordering;

pub fn quick_sort(input_list: &[i32]) -> Vec<i32> {
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
}
