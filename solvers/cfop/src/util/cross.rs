use kubesolv_solvers::{
    cube::{moves::NON_ROTATION_MOVES, state::CubeState},
};

use crate::util::enumerate::enumerate_states_with_criteria;

pub fn cross_solved(state: &CubeState) -> bool {
    // First get color of bottom center
    let bottom_center_color = state.faces[5][4];

    // Check if bottom edges are the same color as bottom center
    let bottom_edge_positions = [1, 3, 5, 7]; // positions of edge stickers on bottom face
    for &pos in &bottom_edge_positions {
        if state.faces[5][pos] != bottom_center_color {
            return false;
        }
    }
    for face_idx in 1..5 {
        let edge_color = state.faces[face_idx][7];
        let center_color = state.faces[face_idx][4];
        if edge_color != center_color {
            return false;
        }
    }
    true
}

pub fn enumerate_cross_solutions(
    state: &kubesolv_solvers::cube::state::CubeState,
    max_depth: usize,
) -> Vec<(kubesolv_solvers::cube::state::CubeState, Vec<kubesolv_solvers::cube::moves::CubeMove>)> {
    let legal_moves = NON_ROTATION_MOVES;
    enumerate_states_with_criteria(state.clone(), max_depth, cross_solved, &legal_moves)
}

#[cfg(test)]
mod tests {
    use kubesolv_solvers::cube::scramble::generate_scramble_sequence;

    use super::*;
    #[test]
    fn test_enumerate_cross_solutions() {
        for i in 0..6 {
            let mut state = CubeState::new_solved();
            let scramble_moves = generate_scramble_sequence(i);
            for mv in scramble_moves.clone() {
                state.execute_move(mv);
            }
            let solutions = enumerate_cross_solutions(&state, i);
            assert!(!solutions.is_empty(), "No cross solutions found");
        }
    }
}
