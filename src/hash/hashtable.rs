#[derive(Clone, Copy)]
struct Bucket<T: Default + Copy + Sized, const BUCKET_SIZE: usize> {
    pub hashes: [u64; BUCKET_SIZE],
    pub values: [T; BUCKET_SIZE]
}

// why
impl<T: Default + Copy + Sized, const BUCKET_SIZE: usize> Default for Bucket<T, BUCKET_SIZE> {
    fn default() -> Self {
        Self {
            hashes: [u64::default(); BUCKET_SIZE],
            values: [T::default(); BUCKET_SIZE]
        }
    }
}

impl<T: Default + Copy + Sized, const BUCKET_SIZE: usize> Bucket<T, BUCKET_SIZE> {
    pub fn clear(&mut self) {
        self.hashes = [0; BUCKET_SIZE];
    }
}

pub struct HashTable<T: Default + Copy + Sized, const COMP_BITS: usize, const BUCKET_SIZE: usize> {
    // AoS -> SoA helps?
    table: Box<[Bucket<T, BUCKET_SIZE>]>
}

impl<T: Default + Copy + Sized, const COMP_BITS: usize, const BUCKET_SIZE: usize> HashTable<T, COMP_BITS, BUCKET_SIZE> {

    // using hash 0 to mean no entry
    // if a hash actually is 0, we get kinda screwed
    pub fn new() -> Self {
        Self {
            table: vec![Bucket::<T, BUCKET_SIZE>::default(); 2usize.pow(COMP_BITS as u32)].into_boxed_slice()
        }
    }

    fn get_bucket(&mut self, hash: u64) -> &mut Bucket<T, BUCKET_SIZE> {
        &mut self.table[(hash & u64::MAX >> (64 - COMP_BITS)) as usize]
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
    
    pub fn set(&mut self, hash: u64, val: T) {

        let bucket = self.get_bucket(hash);

        bucket.hashes[0] = hash;
        bucket.values.rotate_right(1);
        bucket.values[0] = val;
        
    }

    pub fn clear(&mut self) {
        for bucket in self.table.iter_mut() {
            bucket.clear();
        }
    }
}
