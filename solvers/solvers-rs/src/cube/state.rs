// rubiks cube simulator with bitpacked state representation

use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CubeState {
    // Each face of the cube is represented by 9 u8 values (colors)
    // Face layout: 0=U(Up), 1=L(Left), 2=F(Front), 3=R(Right), 4=B(Back), 5=D(Down)
    pub faces: [[u8; 9]; 6],
}

impl Hash for CubeState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for face in &self.faces {
            for sticker in face {
                sticker.hash(state);
            }
        }
    }
}

impl CubeState {
    // Initialize a solved cube
    pub fn new_solved() -> Self {
        CubeState {
            faces: [
                [0; 9], // Face 0
                [1; 9], // Face 1
                [2; 9], // Face 2
                [3; 9], // Face 3
                [4; 9], // Face 4
                [5; 9], // Face 5
            ],
        }
    }

    // Get a color name for display
    fn color_name(color: u8) -> &'static str {
        match color {
            0 => "W",  // White (Up)
            1 => "O",  // Orange (Left)
            2 => "G",  // Green (Front)
            3 => "R",  // Red (Right)
            4 => "B",  // Blue (Back)
            5 => "Y",  // Yellow (Down)
            _ => "?",  // Unknown color (for debugging with marker values)
        }
    }

    // Get a colored block character for display
    pub fn color_block(color: u8) -> &'static str {
        match color {
            0 => "⬜",  // White (Up)
            1 => "🟧",  // Orange (Left)
            2 => "🟩",  // Green (Front)
            3 => "🟥",  // Red (Right)
            4 => "🟦",  // Blue (Back)
            5 => "🟨",  // Yellow (Down)
            _ => "⬛",  // Unknown color (black)
        }
    }

    pub fn format_colored(&self) -> String {
        let mut result = String::new();

        // Top: Up face (White) - 3 rows, aligned with Green
        // Use transparent blocks (⬛ or invisible chars) that have same width as emoji
        for row in 0..3 {
            result.push_str("      ");  // 6 regular spaces
            // Use 3 invisible/transparent blocks to match orange emoji width
            result.push_str("　　　");  // 3 ideographic spaces (double-width)
            result.push(' ');           // 1 space (the gap between orange and green)
            for col in 0..3 {
                result.push_str(Self::color_block(self.faces[0][row * 3 + col]));
            }
            result.push('\n');
        }
        
        // Middle: Left, Front, Right, Back - 3 rows
        for row in 0..3 {
            result.push_str("      ");  // 6 spaces for alignment
            // Left face (3 stickers)
            for col in 0..3 {
                result.push_str(Self::color_block(self.faces[1][row * 3 + col]));
            }
            result.push(' ');  // Space between faces
            // Front face (3 stickers)
            for col in 0..3 {
                result.push_str(Self::color_block(self.faces[2][row * 3 + col]));
            }
            result.push(' ');  // Space between faces
            // Right face (3 stickers)
            for col in 0..3 {
                result.push_str(Self::color_block(self.faces[3][row * 3 + col]));
            }
            result.push(' ');  // Space between faces
            // Back face (3 stickers)
            for col in 0..3 {
                result.push_str(Self::color_block(self.faces[4][row * 3 + col]));
            }
            result.push('\n');
        }
        
        // Bottom: Down face (Yellow) - 3 rows, aligned with Green
        for row in 0..3 {
            result.push_str("      ");  // 6 regular spaces
            // Use 3 invisible/transparent blocks to match orange emoji width
            result.push_str("　　　");  // 3 ideographic spaces (double-width)
            result.push(' ');           // 1 space (the gap between orange and green)
            for col in 0..3 {
                result.push_str(Self::color_block(self.faces[5][row * 3 + col]));
            }
            result.push('\n');
        }
        
        result
    }
}

impl fmt::Display for CubeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Face indices: 0=U(Up/White), 1=L(Left/Orange), 2=F(Front/Green), 
        // 3=R(Right/Red), 4=B(Back/Blue), 5=D(Down/Yellow)
        let u_face = self.faces[0];
        let l_face = self.faces[1];
        let f_face = self.faces[2];
        let r_face = self.faces[3];
        let b_face = self.faces[4];
        let d_face = self.faces[5];

        // Top row: Up face (White, 3x3) - above Green
        for row in 0..3 {
            write!(f, "   ")?;  // 3 spaces for alignment
            for col in 0..3 {
                write!(f, "{}", Self::color_name(u_face[row * 3 + col]))?;
            }
            writeln!(f)?;
        }

        // Middle rows: Left, Front, Right (each 3 chars wide)
        for row in 0..3 {
            for col in 0..3 {
                write!(f, "{}", Self::color_name(l_face[row * 3 + col]))?;
            }
            for col in 0..3 {
                write!(f, "{}", Self::color_name(f_face[row * 3 + col]))?;
            }
            for col in 0..3 {
                write!(f, "{}", Self::color_name(r_face[row * 3 + col]))?;
            }
            writeln!(f)?;
        }

        // Bottom row: Down face (Yellow, 3x3) - below Green
        for row in 0..3 {
            write!(f, "   ")?;  // 3 spaces for alignment
            for col in 0..3 {
                write!(f, "{}", Self::color_name(d_face[row * 3 + col]))?;
            }
            writeln!(f)?;
        }

        // Back face (Blue, 3x3) - below Yellow
        for row in 0..3 {
            write!(f, "   ")?;  // 3 spaces for alignment
            for col in 0..3 {
                write!(f, "{}", Self::color_name(b_face[row * 3 + col]))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::moves::CubeMove;

    use super::*;
    
    #[test]
    fn test_cube_initialization() {
        let cube = CubeState::new_solved();
        for (i, face) in cube.faces.iter().enumerate() {
            for &color in face.iter() {
                assert_eq!(color, i as u8);
            }
        }
    }

    #[test]
    fn test_cube_rotation() {
        // Test that moves work - using F move as example
        let mut cube = CubeState::new_solved();
        
        // Do F move 4 times - should return to solved
        cube.execute_move(CubeMove::F);
        cube.execute_move(CubeMove::F);
        cube.execute_move(CubeMove::F);
        cube.execute_move(CubeMove::F);
        
        let solved = CubeState::new_solved();
        assert_eq!(cube, solved, "4× F should return to solved state");
    }

    #[test]
    fn test_cube_display() {
        let cube = CubeState::new_solved();
        println!("\nSolved cube:");
        println!("{}", cube);
        println!("\nColored display:");
        println!("{}", cube.format_colored());
    }

    #[test]
    fn test_rotate_z_orientation() {
        // Create a cube with distinct values on U, R, D, L to track orientation
        let mut cube = CubeState::new_solved();
        for i in 0..9 {
            cube.faces[0][i] = 10 + i as u8; // U
            cube.faces[3][i] = 20 + i as u8; // R
            cube.faces[5][i] = 30 + i as u8; // D
            cube.faces[1][i] = 40 + i as u8; // L
        }

        let old_u = cube.faces[0].clone();
        let old_r = cube.faces[3].clone();
        let old_d = cube.faces[5].clone();
        let old_l = cube.faces[1].clone();

        // perform rotate_z (90° clockwise around F)
        cube.execute_move(CubeMove::Z);

        // helper: rotate a face 90° CW (same mapping as rotate_face_cw)
        let rotate_cw = |f: [u8;9]| -> [u8;9] {
            [f[6], f[3], f[0], f[7], f[4], f[1], f[8], f[5], f[2]]
        };

        // after rotate_z: old U -> R (rotated CW)
        assert_eq!(cube.faces[3], rotate_cw(old_u));
        // old R -> D
        assert_eq!(cube.faces[5], rotate_cw(old_r));
        // old D -> L
        assert_eq!(cube.faces[1], rotate_cw(old_d));
        // old L -> U
        assert_eq!(cube.faces[0], rotate_cw(old_l));
    }
}
