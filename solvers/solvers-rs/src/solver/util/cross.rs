use crate::{
    cube::{moves::NON_ROTATION_MOVES, state::CubeState},
    solver::util::enumerate::enumerate_states_with_criteria,
};

impl CubeState {
    pub fn cross_solved(&self) -> bool {
        // First get color of bottom center
        let bottom_center_color = self.faces[5][4];

        // Check if bottom edges are the same color as bottom center
        let bottom_edge_positions = [1, 3, 5, 7]; // positions of edge stickers on bottom face
        for &pos in &bottom_edge_positions {
            if self.faces[5][pos] != bottom_center_color {
                return false;
            }
        }
        for face_idx in 1..5 {
            let edge_color = self.faces[face_idx][7];
            let center_color = self.faces[face_idx][4];
            if edge_color != center_color {
                return false;
            }
        }
        true
    }
}

fn cross_solved(state: &CubeState) -> bool {
    state.cross_solved()
}

pub fn enumerate_cross_solutions(
    state: &crate::cube::state::CubeState,
    max_depth: usize,
) -> Vec<(CubeState, Vec<crate::cube::moves::CubeMove>)> {
    let legal_moves = NON_ROTATION_MOVES;
    enumerate_states_with_criteria(state.clone(), max_depth, cross_solved, &legal_moves)
}

#[cfg(test)]
mod tests {
    use crate::cube::scramble::generate_scramble_sequence;

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
