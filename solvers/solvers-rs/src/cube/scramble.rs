use rand::{rng, seq::IndexedRandom};

use crate::cube::{face::move_to_face, moves::NON_ROTATION_MOVES, plane::move_to_plane};

use super::moves::CubeMove;

pub fn generate_scramble_sequence(length: usize) -> Vec<CubeMove> {
    let mut rng = rng();
    let mut scramble = Vec::with_capacity(length);

    for _ in 0..length {
        loop {
            let mv = *NON_ROTATION_MOVES.choose(&mut rng).unwrap();
            // Scroll back in scramble until we get either to the beginning or a move on a different plane.
            let mut mv_works = true;
            for prev in scramble.iter().rev() {
                // if they are not in the same plane, we can stop checking
                if move_to_plane(*prev) != move_to_plane(mv) {
                    break;
                }
                else if move_to_face(*prev) == move_to_face(mv) {
                    // same face, cannot use
                    mv_works = false;
                    continue;
                }
            }
            if mv_works {
                scramble.push(mv);
                break;
            }
        }
    }

    scramble
}
