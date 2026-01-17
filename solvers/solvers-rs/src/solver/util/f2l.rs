use itertools::Itertools;

use crate::{cube::{moves::NON_ROTATION_MOVES, state::CubeState}, solver::util::enumerate::enumerate_states_with_criteria};

pub enum F2LPair {
    FrontRight,
    FrontLeft,
    BackRight,
    BackLeft,
}

impl CubeState {
    pub fn f2l_solved(&self) -> bool {
        self.cross_solved()
            && self.front_right_f2l_pair_solved()
            && self.front_left_f2l_pair_solved()
            && self.back_right_f2l_pair_solved()
            && self.back_left_f2l_pair_solved()
    }
    pub fn front_right_f2l_pair_solved(&self) -> bool {
        // Check if bottom corner is correct
        let bottom_center_color = self.faces[5][4];
        if self.faces[5][2] != bottom_center_color {
            return false;
        }

        // Check if right edge and corner are correct
        let right_center_color = self.faces[3][4];
        if self.faces[3][6] != right_center_color || self.faces[3][3] != right_center_color {
            return false;
        }

        // Check if front edge and corner are correct
        let front_center_color = self.faces[2][4];
        if self.faces[2][8] != front_center_color || self.faces[2][5] != front_center_color {
            return false;
        }
        true
    }

    pub fn front_left_f2l_pair_solved(&self) -> bool {
        // Check if bottom corner is correct
        let bottom_center_color = self.faces[5][4];
        if self.faces[5][0] != bottom_center_color {
            return false;
        }

        // Check if left edge and corner are correct
        let left_center_color = self.faces[1][4];
        if self.faces[1][8] != left_center_color || self.faces[1][5] != left_center_color {
            return false;
        }

        // Check if front edge and corner are correct
        let front_center_color = self.faces[2][4];
        if self.faces[2][6] != front_center_color || self.faces[2][3] != front_center_color {
            return false;
        }
        true
    }

    pub fn back_right_f2l_pair_solved(&self) -> bool {
        // Check if bottom corner is correct
        let bottom_center_color = self.faces[5][4];
        if self.faces[5][8] != bottom_center_color {
            return false;
        }

        // Check if right edge and corner are correct
        let right_center_color = self.faces[3][4];
        if self.faces[3][8] != right_center_color || self.faces[3][5] != right_center_color {
            return false;
        }

        // Check if back edge and corner are correct
        let back_center_color = self.faces[4][4];
        if self.faces[4][6] != back_center_color || self.faces[4][3] != back_center_color {
            return false;
        }
        true
    }

    pub fn back_left_f2l_pair_solved(&self) -> bool {
        // Check if bottom corner is correct
        let bottom_center_color = self.faces[5][4];
        if self.faces[5][6] != bottom_center_color {
            return false;
        }

        // Check if left edge and corner are correct
        let left_center_color = self.faces[1][4];
        if self.faces[1][6] != left_center_color || self.faces[1][3] != left_center_color {
            return false;
        }

        // Check if back edge and corner are correct
        let back_center_color = self.faces[4][4];
        if self.faces[4][8] != back_center_color || self.faces[4][5] != back_center_color {
            return false;
        }
        true
    }
}

fn front_right_f2l_pair_solved(state: &CubeState) -> bool {
    state.front_right_f2l_pair_solved()
}

fn front_left_f2l_pair_solved(state: &CubeState) -> bool {
    state.front_left_f2l_pair_solved()
}

fn back_right_f2l_pair_solved(state: &CubeState) -> bool {
    state.back_right_f2l_pair_solved()
}

fn back_left_f2l_pair_solved(state: &CubeState) -> bool {
    state.back_left_f2l_pair_solved()
}

// Generate criterion function based on boolean mask of which pairs should be solved
pub fn make_criterion(mask: [bool; 4]) -> fn(&CubeState) -> bool {
    match mask {
        [true, false, false, false] => {
            |s: &CubeState| s.cross_solved() && s.front_right_f2l_pair_solved()
        }
        [false, true, false, false] => {
            |s: &CubeState| s.cross_solved() && s.front_left_f2l_pair_solved()
        }
        [false, false, true, false] => {
            |s: &CubeState| s.cross_solved() && s.back_right_f2l_pair_solved()
        }
        [false, false, false, true] => {
            |s: &CubeState| s.cross_solved() && s.back_left_f2l_pair_solved()
        }
        [true, true, false, false] => |s: &CubeState| {
            s.cross_solved() && s.front_right_f2l_pair_solved() && s.front_left_f2l_pair_solved()
        },
        [true, false, true, false] => |s: &CubeState| {
            s.cross_solved() && s.front_right_f2l_pair_solved() && s.back_right_f2l_pair_solved()
        },
        [true, false, false, true] => |s: &CubeState| {
            s.cross_solved() && s.front_right_f2l_pair_solved() && s.back_left_f2l_pair_solved()
        },
        [false, true, true, false] => |s: &CubeState| {
            s.cross_solved() && s.front_left_f2l_pair_solved() && s.back_right_f2l_pair_solved()
        },
        [false, true, false, true] => |s: &CubeState| {
            s.cross_solved() && s.front_left_f2l_pair_solved() && s.back_left_f2l_pair_solved()
        },
        [false, false, true, true] => |s: &CubeState| {
            s.cross_solved() && s.back_right_f2l_pair_solved() && s.back_left_f2l_pair_solved()
        },
        [true, true, true, false] => |s: &CubeState| {
            s.cross_solved()
                && s.front_right_f2l_pair_solved()
                && s.front_left_f2l_pair_solved()
                && s.back_right_f2l_pair_solved()
        },
        [true, true, false, true] => |s: &CubeState| {
            s.cross_solved()
                && s.front_right_f2l_pair_solved()
                && s.front_left_f2l_pair_solved()
                && s.back_left_f2l_pair_solved()
        },
        [true, false, true, true] => |s: &CubeState| {
            s.cross_solved()
                && s.front_right_f2l_pair_solved()
                && s.back_right_f2l_pair_solved()
                && s.back_left_f2l_pair_solved()
        },
        [false, true, true, true] => |s: &CubeState| {
            s.cross_solved()
                && s.front_left_f2l_pair_solved()
                && s.back_right_f2l_pair_solved()
                && s.back_left_f2l_pair_solved()
        },
        [true, true, true, true] => |s: &CubeState| s.f2l_solved(),
        _ => panic!("Invalid pair combination: {:?}", mask),
    }
}

pub fn enumerate_f2l_solutions(
    state: &crate::cube::state::CubeState,
    max_depth_per_pair: usize,
) -> Option<Vec<crate::cube::moves::CubeMove>> {
    // Placeholder implementation
    // Go through all possible orderings of the 4 f2l pairs
    for ordering in (0..4).permutations(4) {
        println!("{:?}", ordering);
        let mut mask = [false, false, false, false];
        for pair_id in ordering {
            mask[pair_id] = true;
            println!("    {:?}", mask);
            let current_criteria = make_criterion(mask.clone());
            let legal_moves = NON_ROTATION_MOVES;
            let current_pair_solutions = enumerate_states_with_criteria(state.clone(), max_depth_per_pair, current_criteria, &legal_moves);
            if current_pair_solutions.len() > 0 {
                // Add these solutions to the list of possibilities

            } else {
                // None found, don't search this ordering any more
            }
        }
    }

    Some(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_front_right() {
        let mut state = CubeState::new_solved();
        assert!(front_right_f2l_pair_solved(&state));
        for mv in [
            crate::cube::moves::CubeMove::R,
            crate::cube::moves::CubeMove::U,
            crate::cube::moves::CubeMove::RPrime,
        ] {
            state.execute_move(mv);
        }
        assert!(!front_right_f2l_pair_solved(&state));
        assert!(front_left_f2l_pair_solved(&state));
        assert!(back_right_f2l_pair_solved(&state));
        assert!(back_left_f2l_pair_solved(&state));
    }

    #[test]
    fn test_front_left() {
        let mut state = CubeState::new_solved();
        assert!(front_left_f2l_pair_solved(&state));
        for mv in [
            crate::cube::moves::CubeMove::LPrime,
            crate::cube::moves::CubeMove::UPrime,
            crate::cube::moves::CubeMove::L,
        ] {
            state.execute_move(mv);
        }
        assert!(front_right_f2l_pair_solved(&state));
        assert!(!front_left_f2l_pair_solved(&state));
        assert!(back_right_f2l_pair_solved(&state));
        assert!(back_left_f2l_pair_solved(&state));
    }

    #[test]
    fn test_back_right() {
        let mut state = CubeState::new_solved();
        assert!(back_right_f2l_pair_solved(&state));
        for mv in [
            crate::cube::moves::CubeMove::RPrime,
            crate::cube::moves::CubeMove::UPrime,
            crate::cube::moves::CubeMove::R,
        ] {
            state.execute_move(mv);
        }
        assert!(front_right_f2l_pair_solved(&state));
        assert!(front_left_f2l_pair_solved(&state));
        assert!(!back_right_f2l_pair_solved(&state));
        assert!(back_left_f2l_pair_solved(&state));
    }

    #[test]
    fn test_back_left() {
        let mut state = CubeState::new_solved();
        assert!(back_left_f2l_pair_solved(&state));
        for mv in [
            crate::cube::moves::CubeMove::L,
            crate::cube::moves::CubeMove::U,
            crate::cube::moves::CubeMove::LPrime,
        ] {
            state.execute_move(mv);
        }
        assert!(front_right_f2l_pair_solved(&state));
        assert!(front_left_f2l_pair_solved(&state));
        assert!(back_right_f2l_pair_solved(&state));
        assert!(!back_left_f2l_pair_solved(&state));
    }
}
