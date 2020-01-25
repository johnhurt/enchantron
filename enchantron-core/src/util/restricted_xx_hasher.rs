use super::IPointHasher;
use crate::model::IPoint;

const FINISHED_BYTE_LENGTH: u64 = (4 + 2) * 8;

const PRIME_1: u64 = 11_400_714_785_074_694_791;
const PRIME_2: u64 = 14_029_467_366_897_019_727;
const PRIME_3: u64 = 1_609_587_929_392_839_161;
const PRIME_4: u64 = 9_650_029_242_287_828_579;

/// Implementation of XxHash that is not generalized to behave like a rust
/// hasher.  Instead this implementation only accepts 64 bit (or multiples)
/// values, and the number of values it will accept is limitted to 6
#[derive(Default)]
pub struct RestrictedXxHasher {
    index: usize,
    v1: u64,
    v2: u64,
    v3: u64,
    v4: u64,
    hash: u64,
}

impl RestrictedXxHasher {
    fn finish_if_seeded(&mut self) {
        self.index += 1;

        match self.index {
            4 => self.finish(),
            _ => {}
        }
    }

    #[inline(always)]
    fn finish(&mut self) {
        #[inline(always)]
        fn ingest_one_number(mut current_value: u64, mut value: u64) -> u64 {
            value = value.wrapping_mul(PRIME_2);
            current_value = current_value.wrapping_add(value);
            current_value = current_value.rotate_left(31);
            current_value.wrapping_mul(PRIME_1)
        };

        let v1 = PRIME_1.wrapping_add(PRIME_2);
        let v2 = PRIME_2;
        let v3 = 0u64;
        let v4 = 0u64.wrapping_sub(PRIME_1);

        let n1 = self.v1;
        let n2 = self.v2;
        let n3 = self.v3;
        let n4 = self.v4;

        self.v1 = ingest_one_number(v1, n1);
        self.v2 = ingest_one_number(v2, n2);
        self.v3 = ingest_one_number(v3, n3);
        self.v4 = ingest_one_number(v4, n4);

        // The original code pulls out local vars for v[1234]
        // here. Performance tests did not show that to be effective
        // here, presumably because this method is not called in a
        // tight loop.

        let mut hash;

        hash = self.v1.rotate_left(1);
        hash = hash.wrapping_add(self.v2.rotate_left(7));
        hash = hash.wrapping_add(self.v3.rotate_left(12));
        hash = hash.wrapping_add(self.v4.rotate_left(18));

        #[inline(always)]
        fn mix_one(mut hash: u64, mut value: u64) -> u64 {
            value = value.wrapping_mul(PRIME_2);
            value = value.rotate_left(31);
            value = value.wrapping_mul(PRIME_1);
            hash ^= value;
            hash = hash.wrapping_mul(PRIME_1);
            hash.wrapping_add(PRIME_4)
        }

        hash = mix_one(hash, self.v1);
        hash = mix_one(hash, self.v2);
        hash = mix_one(hash, self.v3);
        hash = mix_one(hash, self.v4);

        hash = hash.wrapping_add(FINISHED_BYTE_LENGTH);

        self.hash = hash;
    }
}

impl IPointHasher for RestrictedXxHasher {
    fn seed_u64(&mut self, seed: u64) {
        match self.index {
            0 => self.v1 = seed,
            1 => self.v2 = seed,
            2 => self.v3 = seed,
            3 => self.v4 = seed,
            _ => unimplemented!(),
        }

        self.finish_if_seeded();
    }

    fn seed_i64(&mut self, seed: i64) {
        let seed = u64::from_ne_bytes(seed.to_ne_bytes());

        match self.index {
            0 => self.v1 = seed,
            1 => self.v2 = seed,
            2 => self.v3 = seed,
            3 => self.v4 = seed,
            _ => unimplemented!(),
        }

        self.finish_if_seeded();
    }

    fn hash(&self, ipoint: &IPoint) -> u64 {
        debug_assert!(self.index == 4);
        let mut hash = self.hash;

        for v in [
            u64::from_ne_bytes(ipoint.x.to_ne_bytes()),
            u64::from_ne_bytes(ipoint.y.to_ne_bytes()),
        ]
        .iter()
        {
            let mut k1 = v.wrapping_mul(PRIME_2);
            k1 = k1.rotate_left(31);
            k1 = k1.wrapping_mul(PRIME_1);
            hash ^= k1;
            hash = hash.rotate_left(27);
            hash = hash.wrapping_mul(PRIME_1);
            hash = hash.wrapping_add(PRIME_4);
        }

        // The final intermixing
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(PRIME_2);
        hash ^= hash >> 29;
        hash = hash.wrapping_mul(PRIME_3);
        hash ^= hash >> 32;

        hash
    }
}

#[cfg(test)]
mod test {

    use super::super::DefaultXxHashIPointHasher;
    use super::*;
    use crate::model::IPoint;

    const SEED: u64 = 1053880918482810298u64;
    const SCALE: u64 = 12398u64;
    const OFFSET_X: i64 = 112730570710273074i64;
    const OFFSET_Y: i64 = -102023858501028345i64;

    #[test]
    fn test_consistency() {
        let mut expected = DefaultXxHashIPointHasher::default();
        let mut to_test = RestrictedXxHasher::default();

        expected.seed_u64(SEED);
        expected.seed_u64(SCALE);
        expected.seed_i64(OFFSET_X);
        expected.seed_i64(OFFSET_Y);

        to_test.seed_u64(SEED);
        to_test.seed_u64(SCALE);
        to_test.seed_i64(OFFSET_X);
        to_test.seed_i64(OFFSET_Y);

        let test_point = IPoint::new(-114759108235, 2577929734444);

        assert_eq!(expected.hash(&test_point), to_test.hash(&test_point));
    }
}
