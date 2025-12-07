pub mod cfop_exploration;
pub mod util;

pub trait Solver {
    fn solve(
        &self,
        state: &crate::cube::state::CubeState,
    ) -> Option<Vec<crate::cube::moves::CubeMove>>;
}
