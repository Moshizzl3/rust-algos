pub fn quick_sort(input_list: &[i32]) -> Vec<i32> {
    if input_list.len() < 2 {
        return input_list.to_vec();
    }
    let pivot = input_list[input_list.len() / 2];
    let less: Vec<i32> = input_list.iter().filter(|&&x| x < pivot).copied().collect();
    let greater: Vec<i32> = input_list.iter().filter(|x| **x > pivot).copied().collect();
    let equal: Vec<i32> = input_list
        .iter()
        .filter(|&&x| x == pivot)
        .copied()
        .collect();

    quick_sort(&less)
        .into_iter()
        .chain(equal)
        .chain(quick_sort(&greater))
        .collect()
}

mod test {
    use super::*;

    #[test]
    fn test_quicksort_positive_number() {
        let list = vec![2, 4, 1, 3, 5];
        let result = quick_sort(&list);
        assert_eq!(result, [1, 2, 3, 4, 5])
    }

    #[test]
    fn test_quicksort_positive_number_duplicates() {
        let list = vec![2, 4, 1, 3, 5, 3];
        let result = quick_sort(&list);
        assert_eq!(result, [1, 2, 3, 3, 4, 5])
    }
}
