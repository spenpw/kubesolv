use crate::cube::moves::CubeMove;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeFace {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

pub fn move_to_face(cube_move: CubeMove) -> CubeFace {
    // Return the face associated with a single-face move - only for non-rotation moves
    match cube_move {
        CubeMove::U | CubeMove::UPrime | CubeMove::U2 => CubeFace::Up,
        CubeMove::L | CubeMove::LPrime | CubeMove::L2 => CubeFace::Left,
        CubeMove::F | CubeMove::FPrime | CubeMove::F2 => CubeFace::Front,
        CubeMove::R | CubeMove::RPrime | CubeMove::R2 => CubeFace::Right,
        CubeMove::B | CubeMove::BPrime | CubeMove::B2 => CubeFace::Back,
        CubeMove::D | CubeMove::DPrime | CubeMove::D2 => CubeFace::Down,
        _ => panic!("No single face associated with move {:?}", cube_move),
    }
}