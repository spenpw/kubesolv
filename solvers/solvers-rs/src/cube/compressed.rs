use crate::cube::state::CubeState;

/// Compressed cube state representation using bit-packing
/// Stores 54 stickers with 3 bits each = 162 bits = 21 bytes
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CompressedState {
    data: [u8; 21],
}

impl CompressedState {
    /// Compress a CubeState into 21 bytes
    pub fn from_cube_state(state: &CubeState) -> Self {
        let mut data = [0u8; 21];
        let mut bit_pos = 0;

        // Pack all 54 stickers (6 faces × 9 stickers)
        for face in &state.faces {
            for &sticker in face {
                let color = sticker & 0x07; // Only need 3 bits per color (0-5)

                // Determine which byte and bit position
                let byte_idx = bit_pos / 8;
                let bit_offset = bit_pos % 8;

                // Pack 3 bits into the byte
                if bit_offset <= 5 {
                    // Fits in one byte
                    data[byte_idx] |= color << bit_offset;
                } else {
                    // Spans two bytes
                    data[byte_idx] |= color << bit_offset;
                    data[byte_idx + 1] |= color >> (8 - bit_offset);
                }

                bit_pos += 3;
            }
        }

        CompressedState { data }
    }

    /// Decompress back to CubeState
    pub fn to_cube_state(&self) -> CubeState {
        let mut faces = [[0u8; 9]; 6];
        let mut bit_pos = 0;

        for face_idx in 0..6 {
            for sticker_idx in 0..9 {
                let byte_idx = bit_pos / 8;
                let bit_offset = bit_pos % 8;

                // Extract 3 bits
                let color = if bit_offset <= 5 {
                    // Single byte
                    (self.data[byte_idx] >> bit_offset) & 0x07
                } else {
                    // Spans two bytes
                    let low = self.data[byte_idx] >> bit_offset;
                    let high = self.data[byte_idx + 1] << (8 - bit_offset);
                    (low | high) & 0x07
                };

                faces[face_idx][sticker_idx] = color;
                bit_pos += 3;
            }
        }

        CubeState { faces }
    }

    /// Get raw bytes for serialization
    pub fn as_bytes(&self) -> &[u8; 21] {
        &self.data
    }

    /// Create from raw bytes
    pub fn from_bytes(bytes: [u8; 21]) -> Self {
        CompressedState { data: bytes }
    }
}

/// Even more compressed: Skip the 6 fixed center stickers
/// 48 stickers × 3 bits = 144 bits = 18 bytes
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CompressedStateNoCenter {
    data: [u8; 18],
}

impl CompressedStateNoCenter {
    /// Compress a CubeState into 18 bytes (skipping centers)
    pub fn from_cube_state(state: &CubeState) -> Self {
        let mut data = [0u8; 18];
        let mut bit_pos = 0;

        // Pack all stickers except centers (indices 4 of each face)
        for (_face_idx, face) in state.faces.iter().enumerate() {
            for (sticker_idx, &sticker) in face.iter().enumerate() {
                // Skip center sticker (index 4)
                if sticker_idx == 4 {
                    continue;
                }

                let color = sticker & 0x07;
                let byte_idx = bit_pos / 8;
                let bit_offset = bit_pos % 8;

                if bit_offset <= 5 {
                    data[byte_idx] |= color << bit_offset;
                } else {
                    data[byte_idx] |= color << bit_offset;
                    data[byte_idx + 1] |= color >> (8 - bit_offset);
                }

                bit_pos += 3;
            }
        }

        CompressedStateNoCenter { data }
    }

    /// Decompress back to CubeState (centers are reconstructed)
    pub fn to_cube_state(&self) -> CubeState {
        let mut faces = [[0u8; 9]; 6];
        let mut bit_pos = 0;

        for face_idx in 0..6 {
            for sticker_idx in 0..9 {
                if sticker_idx == 4 {
                    // Reconstruct center - always equals face index
                    faces[face_idx][4] = face_idx as u8;
                    continue;
                }

                let byte_idx = bit_pos / 8;
                let bit_offset = bit_pos % 8;

                let color = if bit_offset <= 5 {
                    (self.data[byte_idx] >> bit_offset) & 0x07
                } else {
                    let low = self.data[byte_idx] >> bit_offset;
                    let high = self.data[byte_idx + 1] << (8 - bit_offset);
                    (low | high) & 0x07
                };

                faces[face_idx][sticker_idx] = color;
                bit_pos += 3;
            }
        }

        CubeState { faces }
    }

    pub fn as_bytes(&self) -> &[u8; 18] {
        &self.data
    }

    pub fn from_bytes(bytes: [u8; 18]) -> Self {
        CompressedStateNoCenter { data: bytes }
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::moves::CubeMove;

    use super::*;

    #[test]
    fn test_compression_roundtrip() {
        let original = CubeState::new_solved();
        let compressed = CompressedState::from_cube_state(&original);
        let decompressed = compressed.to_cube_state();
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_compression_size() {
        let state = CubeState::new_solved();
        let compressed = CompressedState::from_cube_state(&state);
        assert_eq!(compressed.as_bytes().len(), 21);
    }

    #[test]
    fn test_compression_no_center_roundtrip() {
        let original = CubeState::new_solved();
        let compressed = CompressedStateNoCenter::from_cube_state(&original);
        let decompressed = compressed.to_cube_state();
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_compression_no_center_size() {
        let state = CubeState::new_solved();
        let compressed = CompressedStateNoCenter::from_cube_state(&state);
        assert_eq!(compressed.as_bytes().len(), 18);
    }

    #[test]
    fn test_compression_with_moves() {
        let mut cube = CubeState::new_solved();
        cube.execute_move(CubeMove::U);
        cube.execute_move(CubeMove::RPrime);
        cube.execute_move(CubeMove::B);

        let compressed = CompressedState::from_cube_state(&cube);
        let decompressed = compressed.to_cube_state();
        assert_eq!(cube, decompressed);
    }
}
