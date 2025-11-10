pub fn sum_recursive(input_list: &[i32]) -> i32 {
    if input_list.is_empty() {
        return 0;
    }
    let first = input_list[0];
    let rest = &input_list[1..];
    first + sum_recursive(rest)
}

pub fn sum_recursive_match(input_list: &[i32]) -> i32 {
    match input_list {
        [] => 0,
        [first, rest @ ..] => first + sum_recursive_match(rest),
    }
}

pub fn count_recursive(input_list: &[i32]) -> i32 {
    if input_list.is_empty() {
        return 0;
    }

    let rest = &input_list[1..];
    1 + count_recursive(rest)
}

pub fn count_recursive_match(input_list: &[i32]) -> i32 {
    match input_list {
        [] => 0,
        [_, rest @ ..] => 1 + count_recursive_match(rest),
    }
}

pub fn max_value_recursive(input_list: &[i32]) -> i32 {
    if input_list.len() == 1 {
        return input_list[0];
    }

    let first = input_list[0];
    let max_rest = max_value_recursive(&input_list[1..]);

    first.max(max_rest)
}

pub fn max_value_recursive_pattern(input_list: &[i32]) -> i32 {
    match input_list {
        [single] => *single,
        [first, rest @ ..] => {
            let max_rest = max_value_recursive_pattern(rest);
            *first.max(&max_rest)
        }
        [] => panic!("woops"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for sum_recursive
    #[test]
    fn test_sum_recursive_empty() {
        assert_eq!(sum_recursive(&[]), 0);
    }

    #[test]
    fn test_sum_recursive_single_element() {
        assert_eq!(sum_recursive(&[5]), 5);
    }

    #[test]
    fn test_sum_recursive_positive_numbers() {
        assert_eq!(sum_recursive(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_sum_recursive_negative_numbers() {
        assert_eq!(sum_recursive(&[-1, -2, -3]), -6);
    }

    #[test]
    fn test_sum_recursive_mixed_numbers() {
        assert_eq!(sum_recursive(&[10, -5, 3, -2]), 6);
    }

    #[test]
    fn test_sum_recursive_with_zero() {
        assert_eq!(sum_recursive(&[0, 1, 0, 2]), 3);
    }

    // Tests for sum_recursive_match
    #[test]
    fn test_sum_recursive_match_empty() {
        assert_eq!(sum_recursive_match(&[]), 0);
    }

    #[test]
    fn test_sum_recursive_match_single_element() {
        assert_eq!(sum_recursive_match(&[5]), 5);
    }

    #[test]
    fn test_sum_recursive_match_positive_numbers() {
        assert_eq!(sum_recursive_match(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_sum_recursive_match_negative_numbers() {
        assert_eq!(sum_recursive_match(&[-1, -2, -3]), -6);
    }

    #[test]
    fn test_sum_recursive_match_mixed_numbers() {
        assert_eq!(sum_recursive_match(&[10, -5, 3, -2]), 6);
    }

    #[test]
    fn test_sum_recursive_match_with_zero() {
        assert_eq!(sum_recursive_match(&[0, 1, 0, 2]), 3);
    }

    // Test that both functions give same results
    #[test]
    fn test_both_functions_agree() {
        let test_cases = vec![
            vec![],
            vec![1],
            vec![1, 2, 3],
            vec![-5, 10, -3],
            vec![0, 0, 0],
        ];

        for case in test_cases {
            assert_eq!(
                sum_recursive(&case),
                sum_recursive_match(&case),
                "Functions disagree on input: {:?}",
                case
            );
        }
    }

    // Test count recursive
    #[test]
    fn test_count_recursive_positive_numbers() {
        assert_eq!(count_recursive(&[1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_count_recursive_negative_numbers() {
        assert_eq!(count_recursive(&[-1, -2, -3, -4, -5]), 5);
    }

    #[test]
    fn test_count_recursive_empty_list() {
        assert_eq!(count_recursive(&[]), 0);
    }

    #[test]
    fn test_count_recursive_one_element() {
        assert_eq!(count_recursive(&[42]), 1);
    }

    #[test]
    fn test_count_recursive_pattern_positive_numbers() {
        assert_eq!(count_recursive_match(&[1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_count_recursive_pattern_negative_numbers() {
        assert_eq!(count_recursive_match(&[1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_count_recursive_pattern_one_element() {
        assert_eq!(count_recursive_match(&[42]), 1);
    }

    #[test]
    fn test_count_recursive_pattern_empty_list() {
        assert_eq!(count_recursive_match(&[]), 0);
    }

    // Max
    #[test]
    fn test_max_recursive_positive_numbers() {
        assert_eq!(max_value_recursive(&[1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_max_recursive_negative_numbers() {
        assert_eq!(max_value_recursive(&[-1, -2, -3, -4, -5]), -1);
    }

    #[test]
    fn test_max_recursive_one_element() {
        assert_eq!(max_value_recursive(&[42]), 42);
    }

    #[test]
    #[should_panic]
    fn test_max_recursive_empty() {
        max_value_recursive(&[]);
    }

    #[test]
    fn test_max_recursive_pattern_positive_numbers() {
        assert_eq!(max_value_recursive_pattern(&[1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_max_recursive_pattern_negative_numbers() {
        assert_eq!(max_value_recursive_pattern(&[-1, -2, -3, -4, -5]), -1);
    }

    #[test]
    fn test_max_recursive_pattern_one_element() {
        assert_eq!(max_value_recursive_pattern(&[42]), 42);
    }

    #[test]
    #[should_panic]
    fn test_max_recursive_pattern_empty() {
        max_value_recursive_pattern(&[]);
    }
    #[test]
    #[should_panic(expected = "woops")]
    fn test_max_recursive_pattern_empty_check_panic() {
        max_value_recursive_pattern(&[]);
    }
}
