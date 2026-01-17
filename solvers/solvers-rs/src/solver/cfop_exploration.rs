pub struct CfopExplorationSolver {
    cross_max_depth: usize,
    first_f2l_pair_max_depth: usize,
    second_f2l_pair_max_depth: usize,
    third_f2l_pair_max_depth: usize,
    fourth_f2l_pair_max_depth: usize,
}

use itertools::{Itertools, Permutations, min};

use super::util::{
    cross::enumerate_cross_solutions,
    enumerate::enumerate_states_with_criteria,
    f2l::F2LPair,
};
use super::Solver;
use crate::cube::{moves::CubeMove, moves::NON_ROTATION_MOVES, state::CubeState};
use crate::solver::util::cross;
use crate::solver::util::f2l::make_criterion;

impl CfopExplorationSolver {
    pub fn new() -> Self {
        CfopExplorationSolver {
            cross_max_depth: 6,
            first_f2l_pair_max_depth: 5,
            second_f2l_pair_max_depth: 5,
            third_f2l_pair_max_depth: 5,
            fourth_f2l_pair_max_depth: 5,
        }
    }
}

impl Solver for CfopExplorationSolver {
    fn solve(
        &self,
        state: &crate::cube::state::CubeState,
    ) -> Option<Vec<crate::cube::moves::CubeMove>> {

        // Loop through possible orientations (which side is down)
            // Loop through possible cross solutions up to cross_max_depth
                // Loop through possible f2l pair orderings
                    // Loop through pairs in current ordering
                        // Loop through possible f2l solutions up to f2l_pair_max_depth
                            // solve oll
                            // solve pll
                            // solve auf
                        // Choose shortest
                // Choose shortest
            // Choose shortest
        // Choose shortest

        let orientation_prefixes = vec![
            vec![], // yellow cross
            vec![crate::cube::moves::CubeMove::X],  // blue cross
            vec![crate::cube::moves::CubeMove::X, crate::cube::moves::CubeMove::X],  // white cross
            vec![crate::cube::moves::CubeMove::XPrime],  // green cross
            vec![crate::cube::moves::CubeMove::Y],  // red cross
            vec![crate::cube::moves::CubeMove::YPrime],  // orange cross
        ];

        let mut shortest_length = usize::MAX;
        let mut shortest_moves = Vec::new();
        
        for orientation_prefix in orientation_prefixes {
            let mut orientation_moves = Vec::new();
            let mut state_after_orientation = state.clone();
            for mv in orientation_prefix {
                state_after_orientation.execute_move(mv);
                orientation_moves.push(mv);
                // Do NOT increment since orientation moves don't count
            }

            let cross_solutions = enumerate_cross_solutions(&state_after_orientation, self.cross_max_depth);
            let min_solution_length = min(cross_solutions.iter().map(|(_, moves)| moves.len()))?;
            // TODO: Just using the shortest crosses so far for speed, eventually expand this...
            let selected_solutions: Vec<&(CubeState, Vec<CubeMove>)> = cross_solutions.iter().filter(|(_, moves)| moves.len() == min_solution_length).collect();
            // println!("Shortest cross solution found. Shortest move count {}, options at move count: {} of {}", min_solution_length, selected_solutions.len(), cross_solutions.len());

            // TODO narrow it down to the to n cross solutions
            for (state_after_cross, cross_moves) in selected_solutions {
                // println!("Trying identified cross solution ({}) moves", cross_moves.len());
                // DFS through F2L pair orderings
                let mut current_moves = orientation_moves.clone();
                current_moves.extend(cross_moves);
                
            }
        }
        
        if shortest_moves.is_empty() {
            None
        } else {
            Some(shortest_moves)
        }
    }
}
