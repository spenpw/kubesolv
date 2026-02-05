use std::path::PathBuf;

use clap::{Parser, arg, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value="input.txt")]
    input_file: PathBuf,

    #[arg(short, long, default_value="input.txt")]
    output_file: PathBuf,
}

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
    let args = Args::parse();
    println!("Input file: {:?}", args.input_file);
    let state = crate::cube::state::CubeState::new_solved();
    let solution = solver.solve(&state);
    println!("Solution: {:?}", solution);
}
