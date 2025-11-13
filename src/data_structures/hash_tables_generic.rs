use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct MoMap<K, V> {
    bucket_item_count: u32,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> MoMap<K, V>
where
    K: Clone + PartialEq + Hash,
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
        K: Debug,
        V: Debug,
    {
        println!("size of bucket: {}", self.buckets.len());
        println!("amount of items added: {}", self.bucket_item_count);
        println!("amount of items added: {:?}", self.buckets);
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.resize();
        let index: usize = self.hashing_function(&key);
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

    pub fn get(&self, key: &K) -> Option<&V> {
        let index: usize = self.hashing_function(&key);
        self.buckets[index]
            .iter()
            .find(|x| x.0 == *key)
            .map(|x| &x.1)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index: usize = self.hashing_function(&key);
        let bucket = &mut self.buckets[index];

        match bucket.iter().position(|x| x.0 == *key) {
            Some(v) => {
                self.bucket_item_count -= 1;
                Some(bucket.swap_remove(v).1)
            }
            None => None,
        }
    }

    fn hash_with_size(&self, key: &K, size: usize) -> usize {
        let mut hasher = DefaultHasher::new();

        key.hash(&mut hasher);
        let hash_value = hasher.finish() as usize;

        hash_value % size
    }

    fn hashing_function(&self, key: &K) -> usize {
        self.hash_with_size(key, self.buckets.len())
    }

    /// resize when load factor of 0.7 is hit
    fn resize(&mut self) {
        let new_size = self.buckets.len() * 2;

        if self.bucket_item_count as f64 / self.buckets.len() as f64 >= 0.7 {
            let new_buckets: Vec<Vec<(K, V)>> = vec![Vec::new(); new_size];
            let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);
            self.bucket_item_count = 0;
            for buckets in old_buckets {
                for (k, v) in buckets {
                    let index: usize = self.hash_with_size(&k, new_size);
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
        let map: MoMap<String, i32> = MoMap::new();
        assert_eq!(map.bucket_item_count, 0);
        assert_eq!(map.buckets.len(), 8);
    }

    #[test]
    fn test_insert_single_item() {
        let mut map = MoMap::new();
        let result = map.insert("hello".to_string(), 42);
        assert_eq!(result, None); // No previous value
        assert_eq!(map.bucket_item_count, 1);
    }

    #[test]
    fn test_insert_and_get() {
        let mut map = MoMap::new();
        map.insert("hello".to_string(), 42);
        assert_eq!(map.get(&"hello".to_string()), Some(&42));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let map: MoMap<String, i32> = MoMap::new();
        assert_eq!(map.get(&"hello".to_string()), None);
    }

    #[test]
    fn test_insert_updates_existing_key() {
        let mut map = MoMap::new();
        map.insert("hello".to_string(), 42);
        let old_value = map.insert("hello".to_string(), 99);

        assert_eq!(old_value, Some(42)); // Returns old value
        assert_eq!(map.get(&"hello".to_string()), Some(&99)); // New value stored
        assert_eq!(map.bucket_item_count, 1); // Count doesn't increase
    }

    #[test]
    fn test_remove_existing_key() {
        let mut map = MoMap::new();
        map.insert("hello".to_string(), 42);

        let removed = map.remove(&"hello".to_string());
        assert_eq!(removed, Some(42));
        assert_eq!(map.get(&"hello".to_string()), None);
        assert_eq!(map.bucket_item_count, 0);
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut map: MoMap<String, i32> = MoMap::new();
        let removed = map.remove(&"hello".to_string());
        assert_eq!(removed, None);
    }

    #[test]
    fn test_multiple_inserts() {
        let mut map = MoMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("c".to_string(), 3);

        assert_eq!(map.bucket_item_count, 3);
        assert_eq!(map.get(&"a".to_string()), Some(&1));
        assert_eq!(map.get(&"b".to_string()), Some(&2));
        assert_eq!(map.get(&"c".to_string()), Some(&3));
    }

    // Collision handling tests
    #[test]
    fn test_handles_collisions() {
        let mut map = MoMap::new();
        // Insert enough items to likely cause some collisions
        for i in 0..20 {
            map.insert(format!("key{}", i), i);
        }

        // All items should still be retrievable
        for i in 0..20 {
            assert_eq!(map.get(&format!("key{}", i)), Some(&i));
        }
    }

    // Resize tests
    #[test]
    fn test_resize_triggered_at_load_factor() {
        let mut map = MoMap::new();

        // Initial size is 8, load factor 0.7 means resize at 6 items
        assert_eq!(map.buckets.len(), 8);

        for i in 0..6 {
            map.insert(format!("key{}", i), i);
        }
        assert_eq!(map.buckets.len(), 8); // Still 8 buckets

        map.insert("key6".to_string(), 6);
        assert_eq!(map.buckets.len(), 16); // Resized to 16!
    }

    #[test]
    fn test_items_accessible_after_resize() {
        let mut map = MoMap::new();

        // Insert items that will trigger resize
        for i in 0..10 {
            map.insert(format!("key{}", i), i);
        }

        // All items should still be accessible after resize
        for i in 0..10 {
            assert_eq!(map.get(&format!("key{}", i)), Some(&i));
        }
    }

    #[test]
    fn test_multiple_resizes() {
        let mut map = MoMap::new();

        // Insert enough to trigger multiple resizes
        // 8 -> 16 -> 32 -> 64
        for i in 0..50 {
            map.insert(format!("key{}", i), i);
        }

        assert_eq!(map.bucket_item_count, 50);
        assert!(map.buckets.len() >= 64); // Should have resized to at least 64

        // Verify all items are still accessible
        for i in 0..50 {
            assert_eq!(map.get(&format!("key{}", i)), Some(&i));
        }
    }

    // Generic type tests
    #[test]
    fn test_integer_keys() {
        let mut map: MoMap<i32, String> = MoMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
    }

    #[test]
    fn test_tuple_keys() {
        let mut map: MoMap<(i32, i32), String> = MoMap::new();
        map.insert((1, 2), "point".to_string());

        assert_eq!(map.get(&(1, 2)), Some(&"point".to_string()));
        assert_eq!(map.get(&(2, 1)), None);
    }

    #[test]
    fn test_custom_value_types() {
        let mut map: MoMap<String, Vec<i32>> = MoMap::new();
        map.insert("numbers".to_string(), vec![1, 2, 3]);

        assert_eq!(map.get(&"numbers".to_string()), Some(&vec![1, 2, 3]));
    }

    // Edge cases
    #[test]
    fn test_empty_string_key() {
        let mut map = MoMap::new();
        map.insert("".to_string(), 42);
        assert_eq!(map.get(&"".to_string()), Some(&42));
    }

    #[test]
    fn test_update_then_remove() {
        let mut map = MoMap::new();
        map.insert("key".to_string(), 1);
        map.insert("key".to_string(), 2);
        map.insert("key".to_string(), 3);

        assert_eq!(map.bucket_item_count, 1); // Only one key

        let removed = map.remove(&"key".to_string());
        assert_eq!(removed, Some(3)); // Latest value
        assert_eq!(map.bucket_item_count, 0);
    }

    #[test]
    fn test_insert_remove_insert_same_key() {
        let mut map = MoMap::new();

        map.insert("key".to_string(), 1);
        assert_eq!(map.get(&"key".to_string()), Some(&1));

        map.remove(&"key".to_string());
        assert_eq!(map.get(&"key".to_string()), None);

        map.insert("key".to_string(), 2);
        assert_eq!(map.get(&"key".to_string()), Some(&2));
    }

    #[test]
    fn test_many_items() {
        let mut map = MoMap::new();

        // Insert 1000 items
        for i in 0..1000 {
            map.insert(i, i * 2);
        }

        assert_eq!(map.bucket_item_count, 1000);

        // Verify random samples
        assert_eq!(map.get(&0), Some(&0));
        assert_eq!(map.get(&500), Some(&1000));
        assert_eq!(map.get(&999), Some(&1998));

        // Remove some
        map.remove(&500);
        assert_eq!(map.get(&500), None);
        assert_eq!(map.bucket_item_count, 999);
    }
}
