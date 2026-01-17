use crate::cube::{
    moves::{CubeMove, CubeMove::*},
    state::CubeState,
};

pub const AUF_MOVES: &[CubeMove] = &[U, UPrime, U2];

pub fn get_auf_solution(
    state: &crate::cube::state::CubeState,
) -> Result<Vec<(CubeState, Vec<crate::cube::moves::CubeMove>)>, String> {
    if state.solved() {
        return Ok(vec![(state.clone(), vec![])]);
    }
    for &mv in AUF_MOVES {
        let mut test_state = state.clone();
        test_state.execute_move(mv);
        if test_state.solved() {
            return Ok(vec![(test_state, vec![mv])]);
        }
    }
    Err("No AUF solution found".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auf_algorithms_maintain_pll() {
        for &mv in AUF_MOVES {
            let mut state = CubeState::new_solved();
            state.execute_move(mv);
            assert!(state.pll_solved(), "PLL should be solved after applying PLL algorithm");
        }
    }
}
