use crate::cube::{
    moves::{CubeMove, CubeMove::*},
    state::CubeState,
};

impl CubeState {
    pub fn pll_solved(&self) -> bool {
        if !self.oll_solved() {
            return false;
        }
        if self.faces[1][1] != self.faces[1][0] || self.faces[1][1] != self.faces[1][2] {
            return false;
        }
        if self.faces[2][1] != self.faces[2][0] || self.faces[2][1] != self.faces[2][2] {
            return false;
        }
        if self.faces[3][1] != self.faces[3][0] || self.faces[3][1] != self.faces[3][2] {
            return false;
        }
        if self.faces[4][1] != self.faces[4][0] || self.faces[4][1] != self.faces[4][2] {
            return false;
        }
        true
    }
}

pub const PLL_ALGORITHMS: &[&[CubeMove]] = &[
    // PLL Aa
    &[RPrime, F, RPrime, B2, R, FPrime, RPrime, B2, R2],
    // PLL Ab
    &[RPrime, BPrime, R, UPrime, R, D, RPrime, U, R, DPrime, R2, B, R],
    // PLL E
    &[R2, U, FPrime, RPrime, U, R, UPrime, RPrime, U, R, UPrime, RPrime, U, R, UPrime, F, UPrime, R2],
    // PLL F
    &[RPrime, U, R, UPrime, R2, FPrime, UPrime, F, U, R, F, RPrime, FPrime, R2],
    // PLL Ga
    &[R2, U, RPrime, U, RPrime, UPrime, R, UPrime, R2, D, UPrime, RPrime, U, R, DPrime],
    // PLL Gb
    &[D, RPrime, UPrime, R, U, DPrime, R2, U, RPrime, U, R, UPrime, R, UPrime, R2],
    // PLL Gc
    &[R2, UPrime, R, UPrime, R, U, RPrime, U, R2, DPrime, U, R, UPrime, RPrime, D],
    // PLL Gd
    &[R, U, RPrime, UPrime, D, R2, UPrime, R, UPrime, RPrime, U, RPrime, U, R2, DPrime],
    // PLL H
    &[R2, U2, R, U2, R2, U2, R2, U2, R, U2, R2],
    // PLL Ja
    &[RPrime, U, LPrime, U2, R, UPrime, RPrime, U2, R, L],
    // PLL Jb
    &[R, U, RPrime, FPrime, R, U, RPrime, UPrime, RPrime, F, R2, UPrime, RPrime],
    // PLL Na
    &[FPrime, R, U, RPrime, UPrime, RPrime, F, R2, F, UPrime, RPrime, UPrime, R, U, FPrime, RPrime],
    // PLL Nb 
    &[RPrime, U, LPrime, U2, R, UPrime, L, RPrime, U, LPrime, U2, R, UPrime, L],
    // PLL Ra
    &[R, UPrime, RPrime, UPrime, R, U, R, D, RPrime, UPrime, R, DPrime, RPrime, U2, RPrime],
    // PLL Rb
    &[RPrime, U2, R, U2, RPrime, F, R, U, RPrime, UPrime, RPrime, FPrime, R2],
    // PLL T
    &[R, U, RPrime, UPrime, RPrime, F, R2, UPrime, RPrime, UPrime, R, U, RPrime, FPrime],
    // PLL Ua
    &[R, U, RPrime, U, RPrime, UPrime, R2, UPrime, RPrime, U, RPrime, U, R],
    // PLL Ub
    &[RPrime, U, RPrime, UPrime, RPrime, UPrime, RPrime, U, R, U, R2],
    // PLL V
    &[RPrime, U, RPrime, UPrime, R, DPrime, RPrime, D, RPrime, U, DPrime, R2, UPrime, R2, D, R2],
    // PLL Y
    &[F, R, UPrime, RPrime, UPrime, R, U, RPrime, FPrime, R, U, RPrime, UPrime, RPrime, F, R, FPrime],
    // PLL Z
    &[RPrime, UPrime, R, UPrime, R, U, R, UPrime, RPrime, U, R, U, R2, UPrime, RPrime],
];

pub fn get_pll_solution(
    state: &crate::cube::state::CubeState,
) -> Result<Vec<(CubeState, Vec<crate::cube::moves::CubeMove>)>, String> {
    if state.pll_solved() {
        return Ok(vec![(state.clone(), vec![])]);
    }
    for setup_seq in [&[] as &[CubeMove], &[U], &[UPrime], &[U2]] {
        let mut modified_state = state.clone();
        for &mv in setup_seq {
            modified_state.execute_move(mv);
        }
        for &algorithm in PLL_ALGORITHMS {
            let mut test_state = modified_state.clone();
            for &mv in algorithm {
                test_state.execute_move(mv);
            }
            if test_state.pll_solved() {
                let mut full_sequence = setup_seq.to_vec();
                full_sequence.extend_from_slice(algorithm);
                return Ok(vec![(test_state, full_sequence)]);
            }
        }
    }
    Err("No PLL solution found".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pll_algorithms_maintain_oll() {
        for &algorithm in PLL_ALGORITHMS {
            let mut state = CubeState::new_solved();
            for &mv in algorithm {
                state.execute_move(mv);
            }
            assert!(
                state.oll_solved(),
                "PLL algorithm {:?} did not maintain OLL solved",
                algorithm
            );
        }
    }

    #[test]
    fn test_solve_pll_many_cases() {
        // Test all combinations of two PLL algorithms applied to a solved cube

        for &scramble_moves_1 in PLL_ALGORITHMS {
            let mut state = CubeState::new_solved();
            for mv in scramble_moves_1 {
                state.execute_move(*mv);
            }
            for &scramble_moves_2 in PLL_ALGORITHMS {
                let mut state = state.clone();
                for mv in scramble_moves_2 {
                    state.execute_move(*mv);
                }
                let result = get_pll_solution(&state);
                match result {
                    Ok(solutions) => {
                        let (solved_state, solution_moves) = &solutions[0];
                        assert!(
                            solved_state.pll_solved(),
                            "PLL solution did not solve PLL"
                        );
                        let mut test_state = state.clone();
                        for &mv in solution_moves {
                            test_state.execute_move(mv);
                        }
                        assert!(
                            test_state.pll_solved(),
                            "Applying PLL solution moves did not result in PLL solved state"
                        );
                    }
                    Err(e) => panic!("Failed to solve PLL where scramble is {:#?} {:#?}: {}", scramble_moves_1, scramble_moves_2, e),
                }
            }
        }
    }
}
