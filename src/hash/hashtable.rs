const NUM_BITS: usize = 20;
const ARR_SIZE: usize = 2usize.pow(NUM_BITS as u32);
const MASK: u64 = u64::MAX >> (64 - NUM_BITS);
const BUCKET_SIZE: usize = 4;

#[derive(Clone, Copy, Default)]
struct Bucket<T: Default + Copy + Sized> {
    pub hashes: [u64; BUCKET_SIZE],
    pub values: [T; BUCKET_SIZE]
}

impl<T: Default + Copy + Sized> Bucket<T> {
    pub fn clear(&mut self) {
        self.hashes = [0; BUCKET_SIZE];
    }
}

pub struct HashTable<T: Default + Copy + Sized> {
    // AoS -> SoA helps?
    table: Box<[Bucket<T>; ARR_SIZE]>
}

impl<T: Default + Copy + Sized> HashTable<T> {

    // using hash 0 to mean no entry
    // if a hash actually is 0, we get kinda screwed
    pub fn new() -> Self {
        Self {
            table: Box::new([Bucket::default(); ARR_SIZE])
        }
    }

    fn get_bucket(&mut self, hash: u64) -> &mut Bucket<T> {
        &mut self.table[(hash & MASK) as usize]
    }

    pub fn get(&mut self, hash: u64) -> Option<&mut T> {

        let bucket = self.get_bucket(hash);

        for (idx, saved_hash) in bucket.hashes.iter_mut().enumerate() {
            if *saved_hash == hash {
                return Some(&mut bucket.values[idx]);
            }
        }

        None

    }
    
    pub fn set(&mut self, hash: u64, mut val: T) {

        let bucket = self.get_bucket(hash);

        bucket.values.rotate_right(1);
        bucket.values[0] = val;
        
    }

    pub fn clear(&mut self) {
        for bucket in self.table.iter_mut() {
            bucket.clear();
        }
    }
}
