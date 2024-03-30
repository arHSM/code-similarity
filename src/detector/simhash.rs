use xxhash_rust::xxh3::xxh3_128_with_seed;

pub struct Simhash {
    vector: [u64; 128],
}

impl Simhash {
    pub fn finish(&self) -> u128 {
        let mut simhash = 0u128;

        for shift in 0..128 {
            if self.vector[shift] > 0 {
                simhash |= 1 << shift;
            }
        }

        simhash
    }

    pub fn write(&mut self, bytes: &[u8]) {
        let hash = xxh3_128_with_seed(bytes, bytes.len() as u64);

        for shift in 0..128 {
            let bit = (hash >> shift) & 1;

            if bit == 1 {
                self.vector[shift] = self.vector[shift].saturating_add(1);
            } else {
                self.vector[shift] = self.vector[shift].saturating_sub(1);
            }
        }
    }
}

impl Default for Simhash {
    fn default() -> Self {
        Self { vector: [0; 128] }
    }
}
