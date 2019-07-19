use std::hash::{BuildHasher, Hasher};

#[derive(Default, Debug, Clone)]
pub struct SpatialBuildHasher;

#[derive(Default)]
pub struct SpatialHasher {
    bytes: [u8; 8],
    bytes_read: usize,
}

impl Hasher for SpatialHasher {
    fn finish(&self) -> u64 {
        u64::from_be_bytes(self.bytes)
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.bytes_read += 1;
            if self.bytes_read <= 16 {
                continue;
            }
            if self.bytes_read > 24 {
                break;
            }
            self.bytes[self.bytes_read - 17] = *b;
        }
    }
}

impl BuildHasher for SpatialBuildHasher {
    type Hasher = SpatialHasher;

    fn build_hasher(&self) -> Self::Hasher {
        SpatialHasher::default()
    }
}
