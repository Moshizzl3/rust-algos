pub fn quick_sort(input_list: &[i32]) -> Vec<i32> {
    if input_list.len() < 2 {
        return input_list.to_vec();
    }
    let pivot = input_list[input_list.len() / 2];
    let less: Vec<i32> = input_list.iter().filter(|&&x| x < pivot).copied().collect();
    let greater: Vec<i32> = input_list.iter().filter(|x| **x > pivot).copied().collect();

    quick_sort(&less)
        .into_iter()
        .chain(vec![pivot])
        .chain(quick_sort(&greater))
        .collect()
}

