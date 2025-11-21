//simple Huffman algorithm for compressing text
use crate::data_structures::tree::TreeNode;
use std::cmp::Ordering;

use std::collections::BinaryHeap;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn count_frequencies(text: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();

    text.chars()
        .for_each(|c| *freq_map.entry(c).or_insert(0) += 1);

    freq_map
}

pub fn build_huffman_tree(text: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let freq_map = count_frequencies(text);
    if freq_map.is_empty() {
        return None;
    }

    //Create the leafs and add to tree

    let mut heap = BinaryHeap::new();
    for (ch, freq) in freq_map {
        let leaf = TreeNode::new_leaf(ch, freq);
        heap.push(HeapNode { node: leaf });
    }

    // Build tree

    while heap.len() > 1 {
        // pop smallest values of the heap - unwrap for now
        let left = heap.pop().unwrap().node;
        let right = heap.pop().unwrap().node;

        // create the parent
        let combined = left.borrow().frequency + right.borrow().frequency;
        let parent = TreeNode::new_internal(combined, left, right);

        //push back
        heap.push(HeapNode { node: parent });
    }

    //root node
    heap.pop().map(|heap_node| heap_node.node)
}

pub fn generate_codes(root: &Rc<RefCell<TreeNode>>) -> HashMap<char, String> {
    let mut codes = HashMap::new();

    if root.borrow().is_leaf() {
        if let Some(ch) = root.borrow().character {
            codes.insert(ch, "0".to_string());
        }
        return codes;
    }

    generate_codes_helper(root, &mut String::new(), &mut codes);

    codes
}

pub fn generate_codes_helper(
    node: &Rc<RefCell<TreeNode>>,
    current_code: &mut String,
    codes: &mut HashMap<char, String>,
) {
    let borrowed_node = node.borrow();

    if let Some(char) = borrowed_node.character
        && borrowed_node.is_leaf()
    {
        codes.insert(char, current_code.to_owned());
        return;
    }

    let left_node = borrowed_node.left.clone();
    let right_node = borrowed_node.right.clone();
    drop(borrowed_node);

    if let Some(left) = left_node {
        current_code.push('0');
        generate_codes_helper(&left, current_code, codes);
        current_code.pop(); // backtrace
    }

    if let Some(right) = right_node {
        current_code.push('1');
        generate_codes_helper(&right, current_code, codes);
        current_code.pop(); //backtrace
    }
}

pub fn encode(text: &str, codes: &HashMap<char, String>) -> Result<String, String> {
    let mut result = String::new();

    for ch in text.chars() {
        match codes.get(&ch) {
            Some(code) => result.push_str(code),
            None => return Err(format!("Character '{}' not in codes", ch)),
        }
    }

    Ok(result)
}

pub fn decode(encoded: &str, tree: &Rc<RefCell<TreeNode>>) -> Result<String, String> {
    let mut result = String::new();
    let mut current_node = tree.clone();

    if current_node.borrow().is_leaf() {
        if let Some(ch) = current_node.borrow().character {
            for _ in encoded.chars() {
                result.push(ch);
            }
        }
        return Ok(result);
    }

    for bit in encoded.chars() {
        let borrowed = current_node.borrow();
        let left_node = borrowed.left.clone();
        let right_node = borrowed.right.clone();
        drop(borrowed);
        if bit == '0' {
            if let Some(ref left) = left_node {
                current_node = left.clone();
            } else {
                return Err(
                    "Invalid encoded string: tried to go left but no left child".to_string()
                );
            }
        } else if bit == '1' {
            if let Some(ref right) = right_node {
                current_node = right.clone();
            } else {
                return Err(
                    "Invalid encoded string: tried to go right but no right child".to_string(),
                );
            }
        } else {
            return Err(format!("Invalid bit: '{}' (expected '0' or '1')", bit));
        }
        let borrowed = current_node.borrow();

        if let Some(char) = borrowed.character
            && borrowed.is_leaf()
        {
            result.push(char);
            drop(borrowed);
            current_node = tree.clone()
        }
    }

    Ok(result)
}

pub struct HeapNode {
    node: Rc<RefCell<TreeNode>>,
}

impl Ord for HeapNode {
    // Reverse ordering: lower frequencies have higher priority
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .node
            .borrow()
            .frequency
            .cmp(&self.node.borrow().frequency)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.node.borrow().frequency == other.node.borrow().frequency
    }
}

impl Eq for HeapNode {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_frequencies_simple() {
        let text = "hello";
        let freqs = count_frequencies(text);

        assert_eq!(freqs.get(&'h'), Some(&1));
        assert_eq!(freqs.get(&'e'), Some(&1));
        assert_eq!(freqs.get(&'l'), Some(&2));
        assert_eq!(freqs.get(&'o'), Some(&1));
    }

    #[test]
    fn test_count_frequencies_empty() {
        let text = "";
        let freqs = count_frequencies(text);
        assert!(freqs.is_empty());
    }

    #[test]
    fn test_count_frequencies_duplicates() {
        let text = "aaabbc";
        let freqs = count_frequencies(text);

        assert_eq!(freqs.get(&'a'), Some(&3));
        assert_eq!(freqs.get(&'b'), Some(&2));
        assert_eq!(freqs.get(&'c'), Some(&1));
    }

    #[test]
    fn test_build_huffman_tree_empty() {
        let tree = build_huffman_tree("");
        assert!(tree.is_none());
    }

    #[test]
    fn test_build_huffman_tree_single_char() {
        let tree = build_huffman_tree("a").unwrap();
        let borrowed = tree.borrow();

        assert_eq!(borrowed.frequency, 1);
        assert_eq!(borrowed.character, Some('a'));
        assert!(borrowed.is_leaf());
    }

    #[test]
    fn test_build_huffman_tree_creates_root() {
        let tree = build_huffman_tree("hello").unwrap();
        let borrowed = tree.borrow();

        assert_eq!(borrowed.frequency, 5); // Total characters
        assert_eq!(borrowed.character, None); // Root has no character
        assert!(!borrowed.is_leaf()); // Root is not a leaf
    }

    #[test]
    fn test_generate_codes_single_char() {
        let tree = build_huffman_tree("a").unwrap();
        let codes = generate_codes(&tree);

        println!("{:?}", codes);

        // Single character gets empty code or single bit
        assert!(codes.contains_key(&'a'));
        assert_eq!(codes.len(), 1);
    }

    #[test]
    fn test_generate_codes_multiple_chars() {
        let tree = build_huffman_tree("aabbc").unwrap();
        let codes = generate_codes(&tree);

        // Should have codes for a, b, c
        assert!(codes.contains_key(&'a'));
        assert!(codes.contains_key(&'b'));
        assert!(codes.contains_key(&'c'));
        assert_eq!(codes.len(), 3);

        // All codes should only contain '0' and '1'
        for code in codes.values() {
            assert!(code.chars().all(|c| c == '0' || c == '1'));
        }
    }

    #[test]
    fn test_generate_codes_frequency_affects_length() {
        let tree = build_huffman_tree("aaabbc").unwrap();
        let codes = generate_codes(&tree);

        // More frequent character should have shorter or equal code
        let a_len = codes.get(&'a').unwrap().len();
        let c_len = codes.get(&'c').unwrap().len();

        assert!(
            a_len <= c_len,
            "Frequent 'a' should have shorter/equal code than rare 'c'"
        );
    }

    #[test]
    fn test_encode_simple() {
        let tree = build_huffman_tree("ab").unwrap();
        let codes = generate_codes(&tree);
        let encoded = encode("ab", &codes).unwrap();

        // Should be binary string
        assert!(encoded.chars().all(|c| c == '0' || c == '1'));
    }

    #[test]
    fn test_encode_missing_character() {
        let tree = build_huffman_tree("ab").unwrap();
        let codes = generate_codes(&tree);
        let result = encode("abc", &codes);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("'c'"));
    }

    #[test]
    fn test_encode_empty_string() {
        let codes = HashMap::new();
        let encoded = encode("", &codes).unwrap();
        assert_eq!(encoded, "");
    }

    #[test]
    fn test_decode_simple() {
        let text = "hello";
        let tree = build_huffman_tree(text).unwrap();
        let codes = generate_codes(&tree);
        let encoded = encode(text, &codes).unwrap();
        let decoded = decode(&encoded, &tree).unwrap();

        assert_eq!(text, decoded);
    }

    #[test]
    fn test_decode_single_char() {
        let text = "a";
        let tree = build_huffman_tree(text).unwrap();
        let codes = generate_codes(&tree);
        let encoded = encode(text, &codes).unwrap();
        let decoded = decode(&encoded, &tree).unwrap();

        assert_eq!(text, decoded);
    }

    #[test]
    fn test_decode_invalid_bit() {
        let tree = build_huffman_tree("ab").unwrap();
        let result = decode("01x10", &tree);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid bit"));
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let texts = vec![
            "hello world",
            "aaa",
            "abcdefghijklmnopqrstuvwxyz",
            "the quick brown fox jumps over the lazy dog",
            "mississippi",
        ];

        for text in texts {
            let tree = build_huffman_tree(text).unwrap();
            let codes = generate_codes(&tree);
            let encoded = encode(text, &codes).unwrap();
            let decoded = decode(&encoded, &tree).unwrap();

            assert_eq!(text, decoded, "Failed roundtrip for: {}", text);
        }
    }

    #[test]
    fn test_compression_actually_compresses() {
        let text = "aaaaaabbbbccde";
        let tree = build_huffman_tree(text).unwrap();
        let codes = generate_codes(&tree);
        let encoded = encode(text, &codes).unwrap();

        let original_bits = text.len() * 8; // ASCII = 8 bits per char
        let compressed_bits = encoded.len();

        // Should achieve some compression
        assert!(
            compressed_bits < original_bits,
            "Compressed {} bits should be less than original {} bits",
            compressed_bits,
            original_bits
        );
    }

    #[test]
    fn test_huffman_codes_are_prefix_free() {
        let tree = build_huffman_tree("aabbccdd").unwrap();
        let codes = generate_codes(&tree);

        // No code should be a prefix of another
        for (char1, code1) in &codes {
            for (char2, code2) in &codes {
                if char1 != char2 {
                    assert!(
                        !code2.starts_with(code1),
                        "Code for '{}' ({}) is prefix of code for '{}' ({})",
                        char1,
                        code1,
                        char2,
                        code2
                    );
                }
            }
        }
    }

    #[test]
    fn test_heap_node_ordering() {
        let node1 = TreeNode::new_leaf('a', 1);
        let node2 = TreeNode::new_leaf('b', 5);
        let node3 = TreeNode::new_leaf('c', 3);

        let mut heap = BinaryHeap::new();
        heap.push(HeapNode {
            node: node2.clone(),
        });
        heap.push(HeapNode {
            node: node1.clone(),
        });
        heap.push(HeapNode {
            node: node3.clone(),
        });

        // Should pop in order: 1, 3, 5 (smallest first due to reverse ordering)
        assert_eq!(heap.pop().unwrap().node.borrow().frequency, 1);
        assert_eq!(heap.pop().unwrap().node.borrow().frequency, 3);
        assert_eq!(heap.pop().unwrap().node.borrow().frequency, 5);
    }

    #[test]
    fn test_unicode_support() {
        let text = "hello 世界 🦀";
        let tree = build_huffman_tree(text).unwrap();
        let codes = generate_codes(&tree);
        let encoded = encode(text, &codes).unwrap();
        let decoded = decode(&encoded, &tree).unwrap();

        assert_eq!(text, decoded);
    }

    #[test]
    fn test_long_text() {
        let text = "a".repeat(1000) + &"b".repeat(500);
        let tree = build_huffman_tree(&text).unwrap();
        let codes = generate_codes(&tree);
        let encoded = encode(&text, &codes).unwrap();
        let decoded = decode(&encoded, &tree).unwrap();

        assert_eq!(text, decoded);
    }
}
