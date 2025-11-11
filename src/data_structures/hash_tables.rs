use std::str::Chars;

#[derive(Clone)]
struct BucketItem<V> {
    key: String,
    value: V,
}

enum MoMapError {
    CreationError,
}
#[derive(Clone)]
struct MoMap<V> {
    bucket_item_count: u32,
    buckets: Vec<Vec<BucketItem<V>>>,
}

impl<V> MoMap<V> {
    pub fn new() -> Self {
        MoMap {
            bucket_item_count: 0,
            buckets: Vec::with_capacity(8),
        }
    }

    pub fn insert(&mut self, item: BucketItem<V>) -> Result<(), MoMapError> {
        let index: usize = self.hashing_function(item.key.chars());
        self.buckets[index].push(item);
        self.bucket_item_count += 1;
        Ok(())
    }

    fn hashing_function(&self, chars: Chars) -> usize {
        let sum: usize = chars.map(|c| c as usize).sum();

        sum % self.buckets.len()
    }
}
