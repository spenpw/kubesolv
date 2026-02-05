pub fn enumerate_states_with_criteria(
    state: kubesolv_solvers::cube::state::CubeState,
    max_depth: usize,
    criterion: fn(&kubesolv_solvers::cube::state::CubeState) -> bool,
    legal_moves: &[kubesolv_solvers::cube::moves::CubeMove],
) -> Vec<(
    kubesolv_solvers::cube::state::CubeState,
    Vec<kubesolv_solvers::cube::moves::CubeMove>,
)> {
    // Warn if max_depth > 6
    if max_depth > 6 {
        eprintln!(
            "Warning: enumerate_states_with_criteria with max_depth > 6 may take a long time."
        );
    }
    let mut results = Vec::new();
    if criterion(&state) {
        results.push((state.clone(), Vec::new()));
    }
    if max_depth != 0 {
        for mv in legal_moves {
            let mut new_state = state.clone();
            new_state.execute_move(*mv);
            let sub_results =
                enumerate_states_with_criteria(new_state, max_depth - 1, criterion, legal_moves);
            for (res_state, mut res_moves) in sub_results {
                let mut moves = vec![*mv];
                moves.append(&mut res_moves);
                results.push((res_state, moves));
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use kubesolv_solvers::cube::moves::NON_ROTATION_MOVES;
    use kubesolv_solvers::cube::scramble::generate_scramble_sequence;
    use kubesolv_solvers::cube::state::CubeState;

    fn test_enumerate_states_with_criteria_depth_scrambled(
        depth: usize,
        scramble_length: usize,
        expected_min: usize,
        expected_max: usize,
    ) {
        fn solved(state: &CubeState) -> bool {
            state.solved()
        }
        let mut state = CubeState::new_solved();
        let scramble_sequence = generate_scramble_sequence(scramble_length);
        for mv in scramble_sequence.clone() {
            state.execute_move(mv);
        }
        let results = enumerate_states_with_criteria(state, depth, solved, &NON_ROTATION_MOVES);
        assert!(results.len() >= expected_min && results.len() <= expected_max, 
            "For depth {} and scramble length {} (scramble: {:?}), expected between {} and {} solutions, got {} (solutions: {:?})",
            depth, scramble_length, scramble_sequence, expected_min, expected_max, results.len(), results.iter().map(|(_, moves)| moves).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_enumerate_states_with_criteria_depth_1_scramble_1() {
        test_enumerate_states_with_criteria_depth_scrambled(1, 1, 1, 1);
    }

    #[test]
    fn test_enumerate_states_with_criteria_depth_1_scramble_2() {
        test_enumerate_states_with_criteria_depth_scrambled(1, 2, 0, 0);
    }

    #[test]
    fn test_enumerate_states_with_criteria_depth_2_scramble_2() {
        test_enumerate_states_with_criteria_depth_scrambled(2, 2, 1, usize::MAX);
    }
    #[test]
    fn test_enumerate_states_with_criteria_depth_5_scramble_5() {
        test_enumerate_states_with_criteria_depth_scrambled(5, 5, 1, usize::MAX);
    }

    #[test]
    fn test_enumerate_states_with_criteria_depth_4_scramble_5() {
        test_enumerate_states_with_criteria_depth_scrambled(4, 5, 0,usize::MAX);
    }
}
