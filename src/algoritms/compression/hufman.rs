//simple Hufman algorithm for compressing text

use std::collections::HashMap;

pub fn count_frequencies(text: &str) -> HashMap<char, i32> {
    let mut freq_map = HashMap::new();

    text.chars()
        .for_each(|c| *freq_map.entry(c).or_insert(0) += 1);

    freq_map
}
