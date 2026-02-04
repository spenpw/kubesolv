use nissy_solver::light::NissyLightSolver;
use kubesolv_solvers::solver::solver_main;

fn main() {
    solver_main(&NissyLightSolver::new())
}
