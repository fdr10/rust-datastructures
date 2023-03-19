
// Optimal number of bits
// m bits = -(n * ln(p))/ ln(2)^2
pub fn num_bits(size: usize, fp_rate: f64) -> usize {
    let num = -1.0f64 * size as f64 * fp_rate.ln();
    let den = 2.0f64.ln().powf(2.0);
    (num / den).ceil() as usize
}
// given m the number of bits
// n the number of items
// We can compute the optimal number of hashes
pub fn num_hashes(m: usize, n: usize) -> usize {
    ((m as f64 / n as f64) * 2.0f64.ln()).ceil() as usize
}

pub struct BloomFilter {
    bitvec: Vec<u8>,
    hashes: usize,
}

impl BloomFilter {
    pub fn new(size: usize, fp_rate: f64) -> BloomFilter {
        let m = num_bits(size, fp_rate);
        let k = num_hashes(m, size);
        BloomFilter {
            bitvec: vec![0; m],
            hashes: k,
        }
    }

    pub fn insert(&mut self, value: &str) {
        for i in 0..self.hashes {
            let index = fasthash::murmur3::hash32_with_seed(value, i as u32)
                % (self.bitvec.len() as u32 * 8);
            let pos = index as usize;
            self.bitvec[pos / 8] |= 1 << (pos % 8);
        }
    }

    pub fn get(&self, value: &str) -> bool {
        for i in 0..self.hashes {
            let index = fasthash::murmur3::hash32_with_seed(value, i as u32)
                % (self.bitvec.len() as u32 * 8);
            let pos = index as usize;
            if (1 << (pos % 8)) & self.bitvec[pos / 8] == 0 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut filter = BloomFilter::new(2, 0.001);
        filter.insert("test");
        assert!(!filter.get("bar"));
        assert!(!filter.get("foo"));
        assert!(!filter.get("bazz"));
        assert!(filter.get("test"));
    }
}
