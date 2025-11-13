use std::fmt::Debug;
use std::str::Chars;

pub struct MoMap<V> {
    bucket_item_count: u32,
    buckets: Vec<Vec<(String, V)>>,
}

impl<V> MoMap<V>
where
    V: Clone,
{
    pub fn new() -> Self {
        let buckets = vec![Vec::new(); 8];
        MoMap {
            bucket_item_count: 0,
            buckets,
        }
    }

    pub fn bla(&self)
    where
        V: Debug,
    {
        println!("size of bucket: {}", self.buckets.len());
        println!("amount of items added: {}", self.bucket_item_count);
        println!("amount of items added: {:?}", self.buckets);
    }

    pub fn insert(&mut self, key: String, value: V) -> Option<V> {
        self.resize();
        let index: usize = self.hashing_function(key.chars());
        let bucket = &mut self.buckets[index];
        if let Some(existing) = bucket.iter_mut().find(|x| x.0 == key) {
            let old_value = std::mem::replace(&mut existing.1, value);
            Some(old_value)
        } else {
            self.buckets[index].push((key, value));
            self.bucket_item_count += 1;
            None
        }
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        let index: usize = self.hashing_function(key.chars());
        self.buckets[index]
            .iter()
            .find(|x| x.0 == key)
            .map(|x| &x.1)
    }

    pub fn remove(&mut self, key: &str) -> Option<V> {
        let index: usize = self.hashing_function(key.chars());
        let bucket = &mut self.buckets[index];

        match bucket.iter().position(|x| x.0 == key) {
            Some(v) => {
                self.bucket_item_count -= 1;
                Some(bucket.swap_remove(v).1)
            }
            None => None,
        }
    }

    /// multiply with a prime number to get a better distribution. using 31 is the better for optimization:
    /// Bitwise shift and subtraction: The number 31 can be expressed as `(2^{5}-1`.
    /// This allows a compiler to replace the multiplication with a much faster bitwise left shift
    /// and a subtraction: `$31 * i$ becomes (i << 5) - i`.
    fn hash_with_size(&self, chars: Chars, size: usize) -> usize {
        let sum: usize = chars
            .into_iter()
            .fold(0, |hash, c| hash.wrapping_mul(31).wrapping_add(c as usize));

        sum % size
    }

    fn hashing_function(&self, chars: Chars) -> usize {
        self.hash_with_size(chars, self.buckets.len())
    }

    /// resize when load factor of 0.7 is hit
    fn resize(&mut self) {
        let new_size = self.buckets.len() * 2;

        if self.bucket_item_count as f64 / self.buckets.len() as f64 >= 0.7 {
            let new_buckets: Vec<Vec<(String, V)>> = vec![Vec::new(); new_size];
            let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);
            self.bucket_item_count = 0;
            for buckets in old_buckets {
                for (k, v) in buckets {
                    let index: usize = self.hash_with_size(k.chars(), new_size);
                    self.buckets[index].push((k, v));
                    self.bucket_item_count += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic operations tests
    #[test]
    fn test_new_map_is_empty() {
        let map: MoMap<i32> = MoMap::new();
        assert_eq!(map.bucket_item_count, 0);
        assert_eq!(map.buckets.len(), 8);
    }

    #[test]
    fn test_insert_single_item() {
        let mut map = MoMap::new();
        let result = map.insert("hello".to_string(), 42);
        assert_eq!(result, None);
        assert_eq!(map.bucket_item_count, 1);
    }

    #[test]
    fn test_insert_and_get() {
        let mut map = MoMap::new();
        map.insert("hello".to_string(), 42);
        assert_eq!(map.get("hello"), Some(&42));
    }

    #[test]
    fn test_get_with_borrowed_str() {
        let mut map = MoMap::new();
        let key = "hello".to_string();
        map.insert(key.clone(), 42);

        // Can use &str for lookup
        assert_eq!(map.get("hello"), Some(&42));
        assert_eq!(map.get(&key), Some(&42));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let map: MoMap<i32> = MoMap::new();
        assert_eq!(map.get("hello"), None);
    }

    #[test]
    fn test_insert_updates_existing_key() {
        let mut map = MoMap::new();
        map.insert("hello".to_string(), 42);
        let old_value = map.insert("hello".to_string(), 99);

        assert_eq!(old_value, Some(42));
        assert_eq!(map.get("hello"), Some(&99));
        assert_eq!(map.bucket_item_count, 1);
    }

    #[test]
    fn test_remove_existing_key() {
        let mut map = MoMap::new();
        map.insert("hello".to_string(), 42);

        let removed = map.remove("hello");
        assert_eq!(removed, Some(42));
        assert_eq!(map.get("hello"), None);
        assert_eq!(map.bucket_item_count, 0);
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut map: MoMap<i32> = MoMap::new();
        let removed = map.remove("hello");
        assert_eq!(removed, None);
    }

    #[test]
    fn test_multiple_inserts() {
        let mut map = MoMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("c".to_string(), 3);

        assert_eq!(map.bucket_item_count, 3);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
        assert_eq!(map.get("c"), Some(&3));
    }

    // String-specific tests
    #[test]
    fn test_similar_strings_different_hashes() {
        let mut map = MoMap::new();
        map.insert("ab".to_string(), 1);
        map.insert("ba".to_string(), 2);

        // "ab" and "ba" should be different keys
        assert_eq!(map.get("ab"), Some(&1));
        assert_eq!(map.get("ba"), Some(&2));
        assert_eq!(map.bucket_item_count, 2);
    }

    #[test]
    fn test_empty_string_key() {
        let mut map = MoMap::new();
        map.insert("".to_string(), 42);
        assert_eq!(map.get(""), Some(&42));
    }

    #[test]
    fn test_long_string_keys() {
        let mut map = MoMap::new();
        let long_key = "a".repeat(1000);
        map.insert(long_key.clone(), 42);
        assert_eq!(map.get(&long_key), Some(&42));
    }

    #[test]
    fn test_unicode_string_keys() {
        let mut map = MoMap::new();
        map.insert("hello".to_string(), 1);
        map.insert("مرحبا".to_string(), 2);
        map.insert("こんにちは".to_string(), 3);
        map.insert("🦀".to_string(), 4);

        assert_eq!(map.get("hello"), Some(&1));
        assert_eq!(map.get("مرحبا"), Some(&2));
        assert_eq!(map.get("こんにちは"), Some(&3));
        assert_eq!(map.get("🦀"), Some(&4));
    }

    #[test]
    fn test_whitespace_in_keys() {
        let mut map = MoMap::new();
        map.insert("hello world".to_string(), 1);
        map.insert("hello  world".to_string(), 2); // Double space
        map.insert(" hello world".to_string(), 3); // Leading space

        assert_eq!(map.get("hello world"), Some(&1));
        assert_eq!(map.get("hello  world"), Some(&2));
        assert_eq!(map.get(" hello world"), Some(&3));
        assert_eq!(map.bucket_item_count, 3);
    }

    // Collision handling tests
    #[test]
    fn test_handles_collisions() {
        let mut map = MoMap::new();
        for i in 0..20 {
            map.insert(format!("key{}", i), i);
        }

        for i in 0..20 {
            assert_eq!(map.get(&format!("key{}", i)), Some(&i));
        }
    }

    // Resize tests
    #[test]
    fn test_resize_triggered_at_load_factor() {
        let mut map = MoMap::new();
        assert_eq!(map.buckets.len(), 8);

        for i in 0..6 {
            map.insert(format!("key{}", i), i);
        }
        assert_eq!(map.buckets.len(), 8);

        map.insert("key6".to_string(), 6);
        assert_eq!(map.buckets.len(), 16);
    }

    #[test]
    fn test_items_accessible_after_resize() {
        let mut map = MoMap::new();

        for i in 0..10 {
            map.insert(format!("key{}", i), i);
        }

        for i in 0..10 {
            assert_eq!(map.get(&format!("key{}", i)), Some(&i));
        }
    }

    #[test]
    fn test_multiple_resizes() {
        let mut map = MoMap::new();

        for i in 0..50 {
            map.insert(format!("key{}", i), i);
        }

        assert_eq!(map.bucket_item_count, 50);
        assert!(map.buckets.len() >= 64);

        for i in 0..50 {
            assert_eq!(map.get(&format!("key{}", i)), Some(&i));
        }
    }

    // Value type tests
    #[test]
    fn test_string_values() {
        let mut map: MoMap<String> = MoMap::new();
        map.insert("name".to_string(), "Alice".to_string());
        assert_eq!(map.get("name"), Some(&"Alice".to_string()));
    }

    #[test]
    fn test_vec_values() {
        let mut map: MoMap<Vec<i32>> = MoMap::new();
        map.insert("numbers".to_string(), vec![1, 2, 3]);
        assert_eq!(map.get("numbers"), Some(&vec![1, 2, 3]));
    }

    #[test]
    fn test_tuple_values() {
        let mut map: MoMap<(i32, String)> = MoMap::new();
        map.insert("data".to_string(), (42, "answer".to_string()));
        assert_eq!(map.get("data"), Some(&(42, "answer".to_string())));
    }

    // Edge cases
    #[test]
    fn test_update_then_remove() {
        let mut map = MoMap::new();
        map.insert("key".to_string(), 1);
        map.insert("key".to_string(), 2);
        map.insert("key".to_string(), 3);

        assert_eq!(map.bucket_item_count, 1);

        let removed = map.remove("key");
        assert_eq!(removed, Some(3));
        assert_eq!(map.bucket_item_count, 0);
    }

    #[test]
    fn test_insert_remove_insert_same_key() {
        let mut map = MoMap::new();

        map.insert("key".to_string(), 1);
        assert_eq!(map.get("key"), Some(&1));

        map.remove("key");
        assert_eq!(map.get("key"), None);

        map.insert("key".to_string(), 2);
        assert_eq!(map.get("key"), Some(&2));
    }

    #[test]
    fn test_many_items() {
        let mut map = MoMap::new();

        for i in 0..1000 {
            map.insert(format!("key{}", i), i * 2);
        }

        assert_eq!(map.bucket_item_count, 1000);

        assert_eq!(map.get("key0"), Some(&0));
        assert_eq!(map.get("key500"), Some(&1000));
        assert_eq!(map.get("key999"), Some(&1998));

        map.remove("key500");
        assert_eq!(map.get("key500"), None);
        assert_eq!(map.bucket_item_count, 999);
    }

    #[test]
    fn test_hash_distribution_quality() {
        let mut map = MoMap::new();

        // Insert items and check they're not all in same bucket
        for i in 0..20 {
            map.insert(format!("test_{}", i), i);
        }

        // Count non-empty buckets
        let non_empty_buckets = map.buckets.iter().filter(|b| !b.is_empty()).count();

        // With good distribution, most buckets should have something
        // (This is probabilistic but should usually pass)
        assert!(
            non_empty_buckets > 1,
            "Poor hash distribution: only {} buckets used",
            non_empty_buckets
        );
    }

    #[test]
    fn test_case_sensitive_keys() {
        let mut map = MoMap::new();
        map.insert("Hello".to_string(), 1);
        map.insert("hello".to_string(), 2);
        map.insert("HELLO".to_string(), 3);

        assert_eq!(map.get("Hello"), Some(&1));
        assert_eq!(map.get("hello"), Some(&2));
        assert_eq!(map.get("HELLO"), Some(&3));
        assert_eq!(map.bucket_item_count, 3);
    }
}
