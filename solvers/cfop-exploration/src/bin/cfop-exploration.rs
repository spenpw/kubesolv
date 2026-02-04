use cfop_exploration_solver::cfop_exploration::CfopExplorationSolver;
use kubesolv_solvers::solver::solver_main;

fn main() {
    solver_main(&CfopExplorationSolver::new())
}
