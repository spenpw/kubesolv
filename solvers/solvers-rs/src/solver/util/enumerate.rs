pub fn enumerate_states_with_criteria(
    state: crate::cube::state::CubeState,
    max_depth: usize,
    criterion: fn(&crate::cube::state::CubeState) -> bool,
    legal_moves: &[crate::cube::moves::CubeMove],
) -> Vec<(
    crate::cube::state::CubeState,
    Vec<crate::cube::moves::CubeMove>,
)> {
    // Sequential version - parallelization adds too much overhead for small search spaces
    let mut results = Vec::new();
    if max_depth == 0 {
        if criterion(&state) {
            results.push((state.clone(), Vec::new()));
        }
    } else {
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
    use crate::cube::moves::NON_ROTATION_MOVES;
    use crate::cube::scramble::generate_scramble_sequence;
    use crate::cube::state::CubeState;

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

    #[test]
    fn test_enumerate_states_with_criteria_depth_6_scramble_6() {
        test_enumerate_states_with_criteria_depth_scrambled(6, 6, 1, usize::MAX);
    }
}
