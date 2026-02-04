pub mod util;

pub trait Solver {
    fn solve(
        &self,
        state: &crate::cube::state::CubeState,
    ) -> Option<Vec<crate::cube::moves::CubeMove>>;
}

pub fn solver_main<S: Solver>(
    solver: &S,
) {
    // TODO figure out how to get the state from the command line or something instead of just using a hardcoded one
    let state = crate::cube::state::CubeState::new_solved();
    let solution = solver.solve(&state);
    println!("Solution: {:?}", solution);
}
