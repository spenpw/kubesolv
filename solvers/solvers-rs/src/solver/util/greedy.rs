
pub fn greedy_find_with_criteria(
    state: crate::cube::state::CubeState,
    max_depth: usize,
    criterion: fn(&crate::cube::state::CubeState) -> bool,
    legal_moves: &[crate::cube::moves::CubeMove],
) -> Option<(
    crate::cube::state::CubeState,
    Vec<crate::cube::moves::CubeMove>,
)> {
    // not implemented yet
    None
}