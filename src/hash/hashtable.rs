const NUM_BITS: usize = 16;
const ARR_SIZE: usize = 2usize.pow(NUM_BITS as u32);

pub struct HashTable<T> {
    mask: u64,
    table: Vec<Vec<(u64, T)>>,
}

impl<T> HashTable<T>
where T: Copy {

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

    pub fn get(&self, hash: u64) -> Option<T> {
        
        let result = &self.table[(hash & self.mask) as usize];

        if result.is_empty() {
            return None;
        }

        for item in result {
            if item.0 == hash {
                return Some(item.1);
            }
        }

        return None;

    }

    pub fn set(&mut self, hash: u64, val: T) {
        self.table[(hash & self.mask) as usize].push((hash, val));
    }

    pub fn clear(&mut self) {
        for vec in self.table.iter_mut() {
            vec.clear();
        }
    }

}