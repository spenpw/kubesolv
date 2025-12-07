use crate::cube::{face::{CubeFace, move_to_face}, moves::CubeMove};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubePlane {
    UpDown,
    LeftRight,
    FrontBack,
}

pub fn move_to_plane(mv: CubeMove) -> CubePlane {
    face_to_plane(move_to_face(mv))
}

pub fn face_to_plane(cube_face: CubeFace) -> CubePlane {
    // Return the face associated with a single-face move - only for non-rotation moves
    match cube_face {
        CubeFace::Up | CubeFace::Down => CubePlane::UpDown,
        CubeFace::Left | CubeFace::Right => CubePlane::LeftRight,
        CubeFace::Front | CubeFace::Back => CubePlane::FrontBack,
    }
}
