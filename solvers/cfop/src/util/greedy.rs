
pub fn greedy_find_with_criteria(
    state: kubesolv_solvers::cube::state::CubeState,
    max_depth: usize,
    criterion: fn(&kubesolv_solvers::cube::state::CubeState) -> bool,
    legal_moves: &[kubesolv_solvers::cube::moves::CubeMove],
) -> Option<(
    kubesolv_solvers::cube::state::CubeState,
    Vec<kubesolv_solvers::cube::moves::CubeMove>,
)> {
    // not implemented yet
    None
}