pub struct NissyOptimalSolver {
}

use kubesolv_solvers::solver::Solver;

impl NissyOptimalSolver {
    pub fn new() -> Self {
        NissyOptimalSolver {
        }
    }
}

impl Solver for NissyOptimalSolver {
    fn solve(
        &self,
        state: &kubesolv_solvers::cube::state::CubeState,
    ) -> Option<Vec<kubesolv_solvers::cube::moves::CubeMove>> {
        todo!()
    }
}
