use std::fmt::Debug;
use std::str::Chars;

#[derive(Debug)]
pub enum MoMapError {
    CreationError,
}
pub struct MoMap<V> {
    bucket_item_count: u32,
    buckets: Vec<Vec<(String, V)>>,
}

impl<V: Clone + Copy + Debug> MoMap<V> {
    pub fn new() -> Self {
        let buckets = vec![Vec::new(); 8];
        MoMap {
            bucket_item_count: 0,
            buckets,
        }
    }

    pub fn bla(&self) {
        println!("size of bucket: {}", self.buckets.len());
        println!("amount of items added: {}", self.bucket_item_count);
        println!("amount of items added: {:?}", self.buckets);
    }

    pub fn insert(&mut self, key: String, value: V) -> Result<(), MoMapError> {
        let index: usize = self.hashing_function(key.chars());
        self.buckets[index].push((key, value));
        self.bucket_item_count += 1;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        let index: usize = self.hashing_function(key.chars());
        self.buckets[index]
            .iter()
            .find(|x| x.0 == key)
            .map(|x| &x.1)
    }

    fn hashing_function(&self, chars: Chars) -> usize {
        let sum: usize = chars.map(|c| c as usize).sum();

        sum % self.buckets.len()
    }
}
