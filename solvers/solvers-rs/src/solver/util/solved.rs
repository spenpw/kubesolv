use crate::cube::state::CubeState;

impl CubeState {
    pub fn solved(&self) -> bool {
        for face in &self.faces {
            let color = face[4];
            if face.iter().any(|&sticker| sticker != color) {
                return false;
            }
        }
        true
    }
}