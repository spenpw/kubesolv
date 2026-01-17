use crate::cube::{
    moves::{CubeMove, CubeMove::*},
    state::CubeState,
};

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

pub const OLL_ALGORITHMS: &[&[CubeMove]] = &[
    // OLL 1
    &[R, U2, R2, F, R, FPrime, U2, RPrime, F, R, FPrime],
    // OLL 2
    &[
        F, R, U, RPrime, UPrime, S, R, U, RPrime, UPrime, FPrime, SPrime,
    ],
    // OLL 3
    &[RPrime, F2, R2, U2, RPrime, F, R, U2, R2, F2, R],
    // OLL 4
    &[RPrime, F2, R2, U2, RPrime, FPrime, R, U2, R2, F2, R],
    // OLL 5
    &[LPrime, B2, R, B, RPrime, B, L],
    // OLL 6
    &[F, UPrime, R2, D, RPrime, UPrime, R, DPrime, R2, U, FPrime],
    // OLL 7
    &[LPrime, U2, L, U2, L, FPrime, LPrime, F],
    // OLL 8
    &[R, U2, RPrime, U2, RPrime, F, R, FPrime],
    // OLL 9
    &[
        R, U, RPrime, UPrime, RPrime, F, R2, U, RPrime, UPrime, FPrime,
    ],
    // OLL 10
    &[R, U, RPrime, U, RPrime, F, R, FPrime, R, U2, RPrime],
    // OLL 11
    &[
        U2, R, U, RPrime, UPrime, RPrime, F, R, FPrime, LPrime, UPrime, L, U, L, FPrime, LPrime, F,
    ],
    // OLL 12
    &[
        F, R, U, RPrime, UPrime, FPrime, U, F, R, U, RPrime, UPrime, FPrime,
    ],
    // OLL 13
    &[F, U, R, U2, RPrime, UPrime, R, U, RPrime, FPrime],
    // OLL 14
    &[RPrime, F, R, U, RPrime, FPrime, R, F, UPrime, FPrime],
    // OLL 15
    &[RPrime, FPrime, R, LPrime, UPrime, L, U, RPrime, F, R],
    // OLL 16
    &[
        RPrime, F, R, U, RPrime, UPrime, FPrime, R, UPrime, RPrime, U2, R,
    ],
    // OLL 17
    &[
        R, U, RPrime, U, RPrime, F, R, FPrime, U2, RPrime, F, R, FPrime,
    ],
    // OLL 18
    &[
        F, RPrime, FPrime, R, U, R, UPrime, RPrime, U, F, R, U, RPrime, UPrime, FPrime,
    ],
    // OLL 19
    &[RPrime, U2, F, R, U, RPrime, UPrime, F2, U2, F, R],
    // OLL 20
    &[
        R, U, RPrime, UPrime, RPrime, F, R, FPrime, R, U2, R2, F, R, FPrime, R, U2, RPrime,
    ],
    // OLL 21
    &[R, U, RPrime, U, R, UPrime, RPrime, U, R, U2, RPrime],
    // OLL 22
    &[R, U2, R2, UPrime, R2, UPrime, R2, U2, R],
    // OLL 23
    &[R2, D, RPrime, U2, R, DPrime, RPrime, U2, RPrime],
    // OLL 24
    &[R, U, R, D, RPrime, UPrime, R, DPrime, R2],
    // OLL 25
    &[R, U2, R, D, RPrime, U2, R, DPrime, R2],
    // OLL 26
    &[RPrime, UPrime, R, UPrime, RPrime, U2, R],
    // OLL 27
    &[R, U, RPrime, U, R, U2, RPrime],
    // OLL 28
    &[F, R, U, RPrime, UPrime, F2, LPrime, UPrime, L, U, F],
    // OLL 29
    &[
        R, U, RPrime, UPrime, R, UPrime, RPrime, FPrime, UPrime, F, R, U, RPrime,
    ],
    // OLL 30
    &[F, U, R, U2, RPrime, UPrime, R, U2, RPrime, UPrime, FPrime],
    // OLL 31
    &[RPrime, UPrime, F, U, R, UPrime, RPrime, FPrime, R],
    // OLL 32
    &[L, U, FPrime, UPrime, LPrime, U, L, F, LPrime],
    // OLL 33
    &[R, U, RPrime, UPrime, RPrime, F, R, FPrime],
    // OLL 34
    &[R, U, R2, UPrime, RPrime, F, R, U, R, UPrime, FPrime],
    // OLL 35
    &[R, U2, R2, F, R, FPrime, R, U2, RPrime],
    // OLL 36
    &[R, U, R2, FPrime, UPrime, F, U, R2, U2, RPrime],
    // OLL 37
    &[F, RPrime, FPrime, R, U, R, UPrime, RPrime],
    // OLL 38
    &[
        R, U, RPrime, U, R, UPrime, RPrime, UPrime, RPrime, F, R, FPrime,
    ],
    // OLL 39
    &[R, U, RPrime, FPrime, UPrime, F, U, R, U2, RPrime],
    // OLL 40
    &[RPrime, F, R, U, RPrime, UPrime, FPrime, U, R],
    // OLL 41
    &[F, U, R2, D, RPrime, UPrime, R, DPrime, R2, FPrime],
    // OLL 42
    &[
        RPrime, UPrime, R, UPrime, RPrime, U2, R, F, R, U, RPrime, UPrime, FPrime,
    ],
    // OLL 43
    &[RPrime, UPrime, FPrime, U, F, R],
    // OLL 44
    &[F, U, R, UPrime, RPrime, FPrime],
    // OLL 45
    &[F, R, U, RPrime, UPrime, FPrime],
    // OLL 46
    &[RPrime, UPrime, RPrime, F, R, FPrime, U, R],
    // OLL 47
    &[FPrime, LPrime, UPrime, L, U, LPrime, UPrime, L, U, F],
    // OLL 48
    &[F, R, U, RPrime, UPrime, R, U, RPrime, UPrime, FPrime],
    // OLL 49
    &[R, BPrime, R2, F, R2, B, R2, FPrime, R],
    // OLL 50
    &[RPrime, F, R2, BPrime, R2, FPrime, R2, B, RPrime],
    // OLL 51
    &[F, U, R, UPrime, RPrime, U, R, UPrime, RPrime, FPrime],
    // OLL 52
    &[RPrime, FPrime, UPrime, F, UPrime, R, U, RPrime, U, R],
    // OLL 53
    &[FPrime, L, F, LPrime, U2, F2, RPrime, FPrime, R, FPrime],
    // OLL 54
    &[F, RPrime, FPrime, R, U2, F2, L, F, LPrime, F],
    // OLL 55
    &[R, U2, R2, UPrime, R, UPrime, RPrime, U2, F, R, FPrime],
    // OLL 56
    &[
        RPrime, FPrime, R, UPrime, LPrime, U, L, UPrime, LPrime, U, L, RPrime, F, R,
    ],
    // OLL 57
    &[LPrime, R, U, RPrime, UPrime, L, RPrime, F, R, FPrime],
];

pub fn get_oll_solution(
    state: &crate::cube::state::CubeState,
) -> Result<Vec<(CubeState, Vec<crate::cube::moves::CubeMove>)>, String> {
    if state.oll_solved() {
        return Ok(vec![(state.clone(), vec![])]);
    }
    for setup_seq in [&[] as &[CubeMove], &[U], &[UPrime], &[U2]] {
        let mut modified_state = state.clone();
        for &mv in setup_seq {
            modified_state.execute_move(mv);
        }
        for &algorithm in OLL_ALGORITHMS {
            let mut test_state = modified_state.clone();
            for &mv in algorithm {
                test_state.execute_move(mv);
            }
            if test_state.oll_solved() {
                let mut full_sequence = setup_seq.to_vec();
                full_sequence.extend_from_slice(algorithm);
                return Ok(vec![(test_state, full_sequence)]);
            }
        }
    }
    Err("No OLL solution found".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oll_algorithms_maintain_f2l() {
        for &algorithm in OLL_ALGORITHMS {
            let mut state = CubeState::new_solved();
            for &mv in algorithm {
                state.execute_move(mv);
            }
            assert!(
                state.f2l_solved(),
                "OLL algorithm {:?} did not maintain F2L solved",
                algorithm
            );
        }
    }

    #[test]
    fn test_solve_oll_many_cases() {
        // Test all combinations of two OLL algorithms applied to a solved cube

        for &scramble_moves_1 in OLL_ALGORITHMS {
            let mut state = CubeState::new_solved();
            for mv in scramble_moves_1 {
                state.execute_move(*mv);
            }
            for &scramble_moves_2 in OLL_ALGORITHMS {
                let mut state = state.clone();
                for mv in scramble_moves_2 {
                    state.execute_move(*mv);
                }
                let result = get_oll_solution(&state);
                match result {
                    Ok(solutions) => {
                        let (solved_state, solution_moves) = &solutions[0];
                        assert!(
                            solved_state.oll_solved(),
                            "OLL solution did not solve the cube"
                        );
                        let mut test_state = state.clone();
                        for &mv in solution_moves {
                            test_state.execute_move(mv);
                        }
                        assert!(
                            test_state.oll_solved(),
                            "Applying OLL solution moves did not result in OLL solved state"
                        );
                    }
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }
}
