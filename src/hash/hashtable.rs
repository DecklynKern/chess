use std::mem::swap;

const NUM_BITS: usize = 20;
const ARR_SIZE: usize = 2usize.pow(NUM_BITS as u32);

pub struct HashTable<T> {
    mask: u64,
    table: Vec<Vec<(u64, T)>>,
}

impl<T> HashTable<T> {

    pub fn new() -> HashTable<T> {
        let mut vec = Vec::with_capacity(ARR_SIZE);
        for _ in 0..ARR_SIZE {
            vec.push(Vec::new());
        };
        HashTable{
            mask: 0xFFFFFFFFFFFFFFFFu64 >> (64 - NUM_BITS),
            table: vec
        }
    }

    pub fn get(&mut self, hash: u64) -> Option<&mut T> {
        
        let result = &mut self.table[(hash & self.mask) as usize];

        for item in result {
            if item.0 == hash {
                return Some(&mut item.1);
            }
        }

        return None;

    }

    pub fn set(&mut self, hash: u64, mut val: T) {
        
        if let Some(item) = self.get(hash) {
            swap(item, &mut val);
            
        } else {
            self.table[(hash & self.mask) as usize].push((hash, val));
        }
    }

    pub fn clear(&mut self) {
        for vec in self.table.iter_mut() {
            vec.clear();
        }
    }

}