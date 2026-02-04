pub struct NissyLightSolver {
}

use kubesolv_solvers::solver::Solver;

impl NissyLightSolver {
    pub fn new() -> Self {
        NissyLightSolver {
        }
    }
}

impl Solver for NissyLightSolver {
    fn solve(
        &self,
        state: &kubesolv_solvers::cube::state::CubeState,
    ) -> Option<Vec<kubesolv_solvers::cube::moves::CubeMove>> {
        todo!()
    }
}
