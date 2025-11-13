use std::fmt::Debug;
use std::str::Chars;

pub struct MoMap<K, V> {
    bucket_item_count: u32,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> MoMap<K, V>
where
    K: Clone + IntoIterator + PartialEq,
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

    pub fn get(&self, key: &K) -> Option<&V> {
        let index: usize = self.hashing_function(key.chars());
        self.buckets[index]
            .iter()
            .find(|x| x.0 == key)
            .map(|x| &x.1)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index: usize = self.hashing_function(key.chars());
        let bucket = &mut self.buckets[index];

        match bucket.iter().position(|x| x.0 == *key) {
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
    fn hash_with_size(&self, chars: K, size: usize) -> usize {
        let sum: usize = chars
            .into_iter()
            .fold(0, |hash, c| hash.wrapping_mul(31).wrapping_add(c as usize));

        sum % size
    }

    fn hashing_function(&self, chars: K) -> usize {
        self.hash_with_size(chars, self.buckets.len())
    }

    /// resize when load factor of 0.7 is hit
    fn resize(&mut self) {
        let new_size = self.buckets.len() * 2;

        if self.bucket_item_count as f64 / self.buckets.len() as f64 >= 0.7 {
            let mut new_buckets: Vec<Vec<(K, V)>> = vec![Vec::new(); new_size];
            self.bucket_item_count = 0;
            self.buckets.clone().iter().for_each(|x| {
                x.clone().iter().for_each(|x| {
                    let clones_value = x.clone();
                    let index: usize = self.hash_with_size(clones_value.0.chars(), new_size);
                    new_buckets[index].push((clones_value.0, clones_value.1));
                    self.bucket_item_count += 1;
                })
            });

            self.buckets = new_buckets
        }
    }
}
