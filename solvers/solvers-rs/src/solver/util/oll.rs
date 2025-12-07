use crate::{cube::{moves::{CubeMove, CubeMove::*, NON_ROTATION_MOVES}, state::CubeState}, solver::util::enumerate::enumerate_states_with_criteria};

impl CubeState {
    pub fn oll_solved(&self) -> bool {
        if !self.f2l_solved() {
            return false;
        }
        let top_color = self.faces[0][4];
        for &pos in &[0, 1, 2, 3, 5, 6, 7, 8] {
            if self.faces[0][pos] != top_color {
                return false;
            }
        }
        true
    }
}

fn oll_solved(state: &CubeState) -> bool {
    state.oll_solved()
}

pub const OLL_ALGORITHMS: &[&[CubeMove]] = &[
    // OLL 1
    &[R, U2, R2, F, R, FPrime, U2, RPrime, F, R, FPrime],
    // OLL 2
    &[F, R, U, RPrime, UPrime, S, R, U, RPrime, UPrime, FPrime, SPrime],
    // OLL 3
    &[UPrime, RPrime, F2, R2, U2, RPrime, FPrime, R, U2, R2, F2, R],
    // OLL 4
    &[RPrime, F2, R2, U2, RPrime, FPrime, R, U2, R2, F2, R],
    // OLL 5
    &[LPrime, B2, R, B, RPrime, B, L],
];


pub fn enumerate_oll_solutions(
    state: &crate::cube::state::CubeState,
    max_depth: usize,
) -> Vec<(CubeState, Vec<crate::cube::moves::CubeMove>)> {
    let legal_moves = NON_ROTATION_MOVES;
    enumerate_states_with_criteria(
        state.clone(),
        max_depth,
        oll_solved,
        &legal_moves,
    )
}

#[cfg(test)]
mod tests {
    use crate::cube::{moves::CubeMove};

    use super::*;
    #[test]
    fn test_enumerate_cross_solutions() {
        let mut state = CubeState::new_solved();
        let scramble_moves = vec![
            CubeMove::R,
            CubeMove::U,
            CubeMove::RPrime,
            CubeMove::LPrime,
            CubeMove::UPrime,
            CubeMove::D,
            CubeMove::R,
        ];
        for mv in scramble_moves.clone() {
            state.execute_move(mv);
        }
        let solutions = enumerate_oll_solutions(&state, 4);
        assert!(!solutions.is_empty(), "No cross solutions found");
    }
}