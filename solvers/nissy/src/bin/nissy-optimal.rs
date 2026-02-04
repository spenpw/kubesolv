use nissy_solver::optimal::NissyOptimalSolver;
use kubesolv_solvers::solver::solver_main;

fn main() {
    solver_main(&NissyOptimalSolver::new())
}
