
use kubesolv_solvers::{cube::{moves::move_sequence_to_standard_notation, scramble::{generate_scramble_sequence}, state::CubeState}, solver::{Solver, cfop_exploration::CfopExplorationSolver}};

fn main() {
    let mut state = CubeState::new_solved();
    let scramble_moves = generate_scramble_sequence(25);
    for mv in scramble_moves.clone() {
        state.execute_move(mv);
    }

    println!("{}", state.format_colored());

    println!("Scramble: {}", move_sequence_to_standard_notation(scramble_moves));

    // Now try to solve it
    CfopExplorationSolver::new().solve(&state);
}
