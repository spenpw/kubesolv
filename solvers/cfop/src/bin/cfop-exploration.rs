use cfop_solvers::cfop_exploration::CfopExplorationSolver;
use kubesolv_solvers::solver::solver_main;

fn main() {
    solver_main(&CfopExplorationSolver::new())
}
