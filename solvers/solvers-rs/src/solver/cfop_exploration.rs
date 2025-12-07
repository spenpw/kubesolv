pub struct CfopExplorationSolver {
    cross_max_depth: usize,
    first_f2l_pair_max_depth: usize,
    second_f2l_pair_max_depth: usize,
    third_f2l_pair_max_depth: usize,
    fourth_f2l_pair_max_depth: usize,
}

use super::util::{
    cross::enumerate_cross_solutions,
    enumerate::enumerate_states_with_criteria,
    f2l::F2LPair,
};
use super::Solver;
use crate::cube::{moves::CubeMove, moves::NON_ROTATION_MOVES, state::CubeState};

impl CfopExplorationSolver {
    pub fn new() -> Self {
        CfopExplorationSolver {
            cross_max_depth: 8,
            first_f2l_pair_max_depth: 5,
            second_f2l_pair_max_depth: 5,
            third_f2l_pair_max_depth: 5,
            fourth_f2l_pair_max_depth: 5,
        }
    }

    // Map pair index to F2LPair enum
    fn pair_index_to_enum(idx: usize) -> F2LPair {
        match idx {
            0 => F2LPair::FrontRight,
            1 => F2LPair::FrontLeft,
            2 => F2LPair::BackRight,
            3 => F2LPair::BackLeft,
            _ => panic!("Invalid pair index"),
        }
    }

    // Check if a specific pair is solved
    fn is_pair_solved(state: &CubeState, pair: &F2LPair) -> bool {
        match pair {
            F2LPair::FrontRight => state.front_right_f2l_pair_solved(),
            F2LPair::FrontLeft => state.front_left_f2l_pair_solved(),
            F2LPair::BackRight => state.back_right_f2l_pair_solved(),
            F2LPair::BackLeft => state.back_left_f2l_pair_solved(),
        }
    }

    // Generate criterion function based on boolean mask of which pairs should be solved
    fn make_criterion(mask: [bool; 4]) -> fn(&CubeState) -> bool {
        match mask {
            [true, false, false, false] => |s: &CubeState| s.cross_solved() && s.front_right_f2l_pair_solved(),
            [false, true, false, false] => |s: &CubeState| s.cross_solved() && s.front_left_f2l_pair_solved(),
            [false, false, true, false] => |s: &CubeState| s.cross_solved() && s.back_right_f2l_pair_solved(),
            [false, false, false, true] => |s: &CubeState| s.cross_solved() && s.back_left_f2l_pair_solved(),
            [true, true, false, false] => |s: &CubeState| s.cross_solved() && s.front_right_f2l_pair_solved() && s.front_left_f2l_pair_solved(),
            [true, false, true, false] => |s: &CubeState| s.cross_solved() && s.front_right_f2l_pair_solved() && s.back_right_f2l_pair_solved(),
            [true, false, false, true] => |s: &CubeState| s.cross_solved() && s.front_right_f2l_pair_solved() && s.back_left_f2l_pair_solved(),
            [false, true, true, false] => |s: &CubeState| s.cross_solved() && s.front_left_f2l_pair_solved() && s.back_right_f2l_pair_solved(),
            [false, true, false, true] => |s: &CubeState| s.cross_solved() && s.front_left_f2l_pair_solved() && s.back_left_f2l_pair_solved(),
            [false, false, true, true] => |s: &CubeState| s.cross_solved() && s.back_right_f2l_pair_solved() && s.back_left_f2l_pair_solved(),
            [true, true, true, false] => |s: &CubeState| s.cross_solved() && s.front_right_f2l_pair_solved() && s.front_left_f2l_pair_solved() && s.back_right_f2l_pair_solved(),
            [true, true, false, true] => |s: &CubeState| s.cross_solved() && s.front_right_f2l_pair_solved() && s.front_left_f2l_pair_solved() && s.back_left_f2l_pair_solved(),
            [true, false, true, true] => |s: &CubeState| s.cross_solved() && s.front_right_f2l_pair_solved() && s.back_right_f2l_pair_solved() && s.back_left_f2l_pair_solved(),
            [false, true, true, true] => |s: &CubeState| s.cross_solved() && s.front_left_f2l_pair_solved() && s.back_right_f2l_pair_solved() && s.back_left_f2l_pair_solved(),
            [true, true, true, true] => |s: &CubeState| s.f2l_solved(),
            _ => panic!("Invalid pair combination: {:?}", mask),
        }
    }

    // DFS through F2L pair orderings
    fn dfs_f2l_orderings(
        &self,
        state: &CubeState,
        moves_so_far: &[CubeMove],
        pairs_solved: &[bool; 4], // FR, FL, BR, BL
        depth: usize,
        best: &mut (usize, Vec<CubeMove>),
    ) {
        // All pairs solved - we have a complete F2L solution
        if pairs_solved.iter().all(|&p| p) {
            let total_len = moves_so_far.len();
            if total_len < best.0 {
                best.0 = total_len;
                best.1 = moves_so_far.to_vec();
            }
            return;
        }

        // Prune if already longer than best
        if moves_so_far.len() >= best.0 {
            return;
        }

        // Try each unsolved pair
        for pair_idx in 0..4 {
            if pairs_solved[pair_idx] {
                continue;
            }

            // Get max depth for this pair based on how many we've solved
            let max_depth = match depth {
                0 => self.first_f2l_pair_max_depth,
                1 => self.second_f2l_pair_max_depth,
                2 => self.third_f2l_pair_max_depth,
                _ => self.fourth_f2l_pair_max_depth,
            };

            // Find solutions for this pair
            let solutions = self.solve_one_pair(state, pairs_solved, pair_idx, max_depth);

            // Try each solution
            for (new_state, pair_moves) in solutions {
                let mut new_moves = moves_so_far.to_vec();
                new_moves.extend(&pair_moves);

                let mut new_pairs_solved = *pairs_solved;
                new_pairs_solved[pair_idx] = true;

                self.dfs_f2l_orderings(&new_state, &new_moves, &new_pairs_solved, depth + 1, best);
            }
        }
    }

    // Solve one F2L pair while keeping previously solved pairs intact
    fn solve_one_pair(
        &self,
        state: &CubeState,
        pairs_solved: &[bool; 4],
        target_pair: usize,
        max_depth: usize,
    ) -> Vec<(CubeState, Vec<CubeMove>)> {
        // Build mask of which pairs should be solved (previously solved + target)
        let mut mask = *pairs_solved;
        mask[target_pair] = true;

        let criterion = Self::make_criterion(mask);
        enumerate_states_with_criteria(state.clone(), max_depth, criterion, NON_ROTATION_MOVES)
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
            
            for (state_after_cross, cross_moves) in cross_solutions {
                // DFS through F2L pair orderings
                let mut current_moves = orientation_moves.clone();
                current_moves.extend(&cross_moves);
                
                let mut best = (usize::MAX, Vec::new());
                self.dfs_f2l_orderings(
                    &state_after_cross,
                    &current_moves,
                    &[false, false, false, false], // No pairs solved yet
                    0,
                    &mut best,
                );
                
                if best.0 < shortest_length {
                    shortest_length = best.0;
                    shortest_moves = best.1;
                }
            }
        }
        
        if shortest_moves.is_empty() {
            None
        } else {
            Some(shortest_moves)
        }
    }
}

