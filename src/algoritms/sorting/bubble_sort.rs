pub fn bubble_sort<T: Clone + Ord>(input_list: &[T]) -> Vec<T> {
    let mut result = input_list.to_vec();
    let n = result.len();

    for i in 0..n {
        for j in 0..(n - i - 1) {
            if result[j] > result[j + 1] {
                result.swap(j, j + 1);
            }
        }
    }

    result
}

mod test {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let list = vec![3, 1, 4, 1, 5];
        let result = bubble_sort(&list);
        assert_eq!(result, [1, 1, 3, 4, 5]);
    }
}
