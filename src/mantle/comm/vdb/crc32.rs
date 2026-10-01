//! CRC-32 checksum calculation matching `core/device/vdb/crc32.hpp` and `crc32.cpp`.

const CRC32_TABLE: [u32; 16] = [
    0x00000000, 0x1db71064, 0x3b6e20c8, 0x26d930ac, 0x76dc4190, 0x6b6b51f4, 0x4db26158, 0x5005713c,
    0xedb88320, 0xf00f9344, 0xd6d6a3e8, 0xcb61b38c, 0x9b64c2b0, 0x86d3d2d4, 0xa00ae278, 0xbdbdf21c,
];

/// Computes IEEE 802.3 CRC32 checksums incrementally using 4-bit nibble lookups.
#[derive(Debug, Clone)]
pub struct Crc32 {
    state: u32,
}

impl Default for Crc32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Crc32 {
    /// Creates a new CRC32 calculator initialized with `!0`.
    pub fn new() -> Self {
        Self { state: !0 }
    }

    /// Resets the checksum calculation state.
    pub fn reset(&mut self) {
        self.state = !0;
    }

    /// Updates the checksum with a single byte.
    pub fn update_byte(&mut self, data: u8) {
        let mut tbl_idx = (self.state as u8) ^ (data & 0x0F);
        self.state = CRC32_TABLE[(tbl_idx & 0x0F) as usize] ^ (self.state >> 4);

        tbl_idx = (self.state as u8) ^ (data >> 4);
        self.state = CRC32_TABLE[(tbl_idx & 0x0F) as usize] ^ (self.state >> 4);
    }

    /// Updates the checksum with a byte slice.
    pub fn update(&mut self, data: &[u8]) {
        for &b in data {
            self.update_byte(b);
        }
    }

    /// Finalizes and returns the 32-bit checksum.
    pub fn finalize(&self) -> u32 {
        !self.state
    }

    /// Helper that calculates the checksum of an entire byte slice.
    pub fn calculate(data: &[u8]) -> u32 {
        let mut crc = Self::new();
        crc.update(data);
        crc.finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32() {
        let checksum = Crc32::calculate(b"123456789");
        assert_eq!(checksum, 0xCBF43926);
    }
}
