/// Binary search
pub fn binary_search(input_list: &[i32], target: i32) -> Option<i32> {
    let mut low: usize = 0;
    let mut high: usize = input_list.len();
    let mut count = 0;

    while low < high {
        let mid = (low + high) / 2;
        let result: i32 = input_list[mid];
        count += 1;
        println!("count is {count}");
        if result == target {
            return Some(input_list[mid]);
        } else if result < target {
            low = mid + 1
        } else {
            high = mid
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found_in_middle() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&list, 3), Some(3))
    }
    #[test]
    fn test_found_at_start() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&list, 1), Some(1))
    }
    #[test]
    fn test_found_at_end() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&list, 5), Some(5))
    }
    #[test]
    fn test_found_at_not_found() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&list, 10), None)
    }
    #[test]
    fn test_found_one_element() {
        let list = vec![42];
        assert_eq!(binary_search(&list, 42), Some(42))
    }

    #[test]
    fn test_found_one_element_not_found() {
        let list = vec![42];
        assert_eq!(binary_search(&list, 1), None)
    }

    #[test]
    fn test_found_at_empty_list() {
        let list = vec![];
        assert_eq!(binary_search(&list, 1), None)
    }
}
