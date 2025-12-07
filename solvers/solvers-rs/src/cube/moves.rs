use super::state::CubeState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeMove {
    U,
    UPrime,
    U2,
    D,
    DPrime,
    D2,
    L,
    LPrime,
    L2,
    R,
    RPrime,
    R2,
    F,
    FPrime,
    F2,
    B,
    BPrime,
    B2,
    M,
    MPrime,
    M2,
    E,
    EPrime,
    E2,
    S,
    SPrime,
    S2,
    X,
    XPrime,
    X2,
    Y,
    YPrime,
    Y2,
    Z,
    ZPrime,
    Z2,
}

pub const NON_ROTATION_MOVES: &[CubeMove] = &[
    CubeMove::U, CubeMove::UPrime, CubeMove::U2,
    CubeMove::D, CubeMove::DPrime, CubeMove::D2,
    CubeMove::L, CubeMove::LPrime, CubeMove::L2,
    CubeMove::R, CubeMove::RPrime, CubeMove::R2,
    CubeMove::F, CubeMove::FPrime, CubeMove::F2,
    CubeMove::B, CubeMove::BPrime, CubeMove::B2,
];


pub fn invert_move(cube_move: CubeMove) -> CubeMove {
    match cube_move {
        CubeMove::U => CubeMove::UPrime,
        CubeMove::UPrime => CubeMove::U,
        CubeMove::D => CubeMove::DPrime,
        CubeMove::DPrime => CubeMove::D,
        CubeMove::L => CubeMove::LPrime,
        CubeMove::LPrime => CubeMove::L,
        CubeMove::R => CubeMove::RPrime,
        CubeMove::RPrime => CubeMove::R,
        CubeMove::F => CubeMove::FPrime,
        CubeMove::FPrime => CubeMove::F,
        CubeMove::B => CubeMove::BPrime,
        CubeMove::BPrime => CubeMove::B,
        CubeMove::M => CubeMove::MPrime,
        CubeMove::MPrime => CubeMove::M,
        CubeMove::E => CubeMove::EPrime,
        CubeMove::EPrime => CubeMove::E,
        CubeMove::S => CubeMove::SPrime,
        CubeMove::SPrime => CubeMove::S,
        CubeMove::X => CubeMove::XPrime,
        CubeMove::XPrime => CubeMove::X,
        CubeMove::Y => CubeMove::YPrime,
        CubeMove::YPrime => CubeMove::Y,
        CubeMove::Z => CubeMove::ZPrime,
        CubeMove::ZPrime => CubeMove::Z,
        CubeMove::U2 => CubeMove::U2,
        CubeMove::D2 => CubeMove::D2,
        CubeMove::L2 => CubeMove::L2,
        CubeMove::R2 => CubeMove::R2,
        CubeMove::F2 => CubeMove::F2,
        CubeMove::B2 => CubeMove::B2,
        CubeMove::M2 => CubeMove::M2,
        CubeMove::E2 => CubeMove::E2,
        CubeMove::S2 => CubeMove::S2,
        CubeMove::X2 => CubeMove::X2,
        CubeMove::Y2 => CubeMove::Y2,
        CubeMove::Z2 => CubeMove::Z2,
    }
}

impl CubeState {
    // Helper function to rotate a single face 90° clockwise
    fn rotate_face_cw(&mut self, face: usize) {
        let f = &mut self.faces[face];
        let temp = f.clone();
        f[0] = temp[6];
        f[1] = temp[3];
        f[2] = temp[0];
        f[3] = temp[7];
        f[4] = temp[4];
        f[5] = temp[1];
        f[6] = temp[8];
        f[7] = temp[5];
        f[8] = temp[2];
    }

    fn rotate_face_ccw(&mut self, face: usize) {
        let f = &mut self.faces[face];
        let temp = f.clone();
        f[0] = temp[2];
        f[1] = temp[5];
        f[2] = temp[8];
        f[3] = temp[1];
        f[4] = temp[4];
        f[5] = temp[7];
        f[6] = temp[0];
        f[7] = temp[3];
        f[8] = temp[6];
    }

    // BEGIN primitives
    fn rotate_x(&mut self) {
        // X rotation: rotate entire cube around R-L axis (like doing R move on whole cube)
        // Looking from R side, clockwise: F → U → B → D → F
        let temp_f = self.faces[2].clone();
        self.faces[2] = self.faces[5].clone(); // F ← D
        self.faces[5] = self.faces[4].clone(); // D ← B
        self.faces[4] = self.faces[0].clone(); // B ← U
        self.faces[0] = temp_f; // U ← F

        // When B and D move, they need to be rotated 180° because they're flipped
        self.rotate_face_cw(4);
        self.rotate_face_cw(4); // B rotated 180°
        self.rotate_face_cw(5);
        self.rotate_face_cw(5); // D rotated 180°

        // Rotate the R and L faces themselves
        self.rotate_face_cw(3); // R rotates CW
        self.rotate_face_ccw(1); // L rotates CCW
    }

    fn rotate_y(&mut self) {
        // Y rotation: rotate entire cube around U-D axis (like doing U move on whole cube)
        // Looking from above, clockwise: F → R → B → L → F
        let temp_f = self.faces[2].clone();
        self.faces[2] = self.faces[3].clone(); // F ← R
        self.faces[3] = self.faces[4].clone(); // R ← B
        self.faces[4] = self.faces[1].clone(); // B ← L
        self.faces[1] = temp_f; // L ← F

        // Rotate the U and D faces themselves
        self.rotate_face_cw(0); // U rotates CW
        self.rotate_face_ccw(5); // D rotates CCW
    }

    fn rotate_z(&mut self) {
        // Z rotation: rotate entire cube around F-B axis (like doing F move on whole cube)
        // Face cycle: U → R → D → L → U
        let temp_u = self.faces[0].clone();
        self.faces[0] = self.faces[1].clone(); // U ← L
        self.faces[1] = self.faces[5].clone(); // L ← D
        self.faces[5] = self.faces[3].clone(); // D ← R
        self.faces[3] = temp_u; // R ← U

        // Rotate the F and B faces themselves
        self.rotate_face_cw(2); // F rotates CW
        self.rotate_face_ccw(4); // B rotates CCW

        // Each face rotates 90° CW as it moves around the cycle
        self.rotate_face_cw(0); // U rotates CW
        self.rotate_face_cw(3); // R rotates CW
        self.rotate_face_cw(5); // D rotates CW
        self.rotate_face_cw(1); // L rotates CW
    }

    fn move_f(&mut self) {
        // Rotate front face clockwise
        self.rotate_face_cw(2);

        // Edge cycle: U_bottom → R_left → D_top → L_right → U_bottom
        let temp = [self.faces[0][6], self.faces[0][7], self.faces[0][8]];

        // U bottom ← L right column (reversed)
        self.faces[0][6] = self.faces[1][8];
        self.faces[0][7] = self.faces[1][5];
        self.faces[0][8] = self.faces[1][2];

        // L right column ← D top row
        self.faces[1][2] = self.faces[5][0];
        self.faces[1][5] = self.faces[5][1];
        self.faces[1][8] = self.faces[5][2];

        // D top row ← R left column (reversed)
        self.faces[5][0] = self.faces[3][6];
        self.faces[5][1] = self.faces[3][3];
        self.faces[5][2] = self.faces[3][0];

        // R left column ← U bottom (from temp)
        self.faces[3][0] = temp[0];
        self.faces[3][3] = temp[1];
        self.faces[3][6] = temp[2];
    }

    fn move_f_prime(&mut self) {
        // Rotate front face counterclockwise
        self.rotate_face_ccw(2);
        // Edge cycle: U_bottom → R_left → D_top → L_right → U_bottom

        let temp = [self.faces[0][6], self.faces[0][7], self.faces[0][8]];
        // U bottom ← R left column
        self.faces[0][6] = self.faces[3][0];
        self.faces[0][7] = self.faces[3][3];
        self.faces[0][8] = self.faces[3][6];
        // R left column ← D top row (reversed)
        self.faces[3][0] = self.faces[5][2];
        self.faces[3][3] = self.faces[5][1];
        self.faces[3][6] = self.faces[5][0];
        // D top row ← L right column
        self.faces[5][0] = self.faces[1][2];
        self.faces[5][1] = self.faces[1][5];
        self.faces[5][2] = self.faces[1][8];
        // L right column ← U bottom (from temp, reversed)
        self.faces[1][2] = temp[2];
        self.faces[1][5] = temp[1];
        self.faces[1][8] = temp[0];
    }
    // END primitives

    pub fn execute_move(&mut self, cube_move: CubeMove) {
        match cube_move {
            CubeMove::U => {
                self.execute_move(CubeMove::XPrime);
                self.execute_move(CubeMove::F);
                self.execute_move(CubeMove::X);
            }
            CubeMove::UPrime => {
                self.execute_move(CubeMove::XPrime);
                self.execute_move(CubeMove::FPrime);
                self.execute_move(CubeMove::X);
            }
            CubeMove::U2 => {
                self.execute_move(CubeMove::XPrime);
                self.execute_move(CubeMove::F2);
                self.execute_move(CubeMove::X);
            }
            CubeMove::D => {
                self.execute_move(CubeMove::X);
                self.execute_move(CubeMove::F);
                self.execute_move(CubeMove::XPrime);
            }
            CubeMove::DPrime => {
                self.execute_move(CubeMove::X);
                self.execute_move(CubeMove::FPrime);
                self.execute_move(CubeMove::XPrime);
            }
            CubeMove::D2 => {
                self.execute_move(CubeMove::X);
                self.execute_move(CubeMove::F2);
                self.execute_move(CubeMove::XPrime);
            }
            CubeMove::L => {
                self.execute_move(CubeMove::YPrime);
                self.execute_move(CubeMove::F);
                self.execute_move(CubeMove::Y);
            }
            CubeMove::LPrime => {
                self.execute_move(CubeMove::YPrime);
                self.execute_move(CubeMove::FPrime);
                self.execute_move(CubeMove::Y);
            }
            CubeMove::L2 => {
                self.execute_move(CubeMove::YPrime);
                self.execute_move(CubeMove::F2);
                self.execute_move(CubeMove::Y);
            }
            CubeMove::R => {
                self.execute_move(CubeMove::Y);
                self.execute_move(CubeMove::F);
                self.execute_move(CubeMove::YPrime);
            }
            CubeMove::RPrime => {
                self.execute_move(CubeMove::Y);
                self.execute_move(CubeMove::FPrime);
                self.execute_move(CubeMove::YPrime);
            }
            CubeMove::R2 => {
                self.execute_move(CubeMove::Y);
                self.execute_move(CubeMove::F2);
                self.execute_move(CubeMove::YPrime);
            }
            CubeMove::F => self.move_f(),
            CubeMove::FPrime => self.move_f_prime(),
            CubeMove::F2 => {
                self.move_f();
                self.move_f();
            }
            CubeMove::B => {
                self.execute_move(CubeMove::Y2);
                self.execute_move(CubeMove::F);
                self.execute_move(CubeMove::Y2);
            }
            CubeMove::BPrime => {
                self.execute_move(CubeMove::Y2);
                self.execute_move(CubeMove::FPrime);
                self.execute_move(CubeMove::Y2);
            }
            CubeMove::B2 => {
                self.execute_move(CubeMove::Y2);
                self.execute_move(CubeMove::F2);
                self.execute_move(CubeMove::Y2);
            }
            CubeMove::M => {
                self.execute_move(CubeMove::XPrime);
                self.execute_move(CubeMove::R);
                self.execute_move(CubeMove::LPrime);
            }
            CubeMove::MPrime => {
                self.execute_move(CubeMove::X);
                self.execute_move(CubeMove::RPrime);
                self.execute_move(CubeMove::L);
            }
            CubeMove::M2 => {
                self.execute_move(CubeMove::X2);
                self.execute_move(CubeMove::R2);
                self.execute_move(CubeMove::L2);
            }
            CubeMove::E => {
                self.execute_move(CubeMove::YPrime);
                self.execute_move(CubeMove::U);
                self.execute_move(CubeMove::DPrime);
            }
            CubeMove::EPrime => {
                self.execute_move(CubeMove::Y);
                self.execute_move(CubeMove::UPrime);
                self.execute_move(CubeMove::D);
            }
            CubeMove::E2 => {
                self.execute_move(CubeMove::Y2);
                self.execute_move(CubeMove::U2);
                self.execute_move(CubeMove::D2);
            }
            CubeMove::S => {
                self.execute_move(CubeMove::Z);
                self.execute_move(CubeMove::FPrime);
                self.execute_move(CubeMove::B);
            }
            CubeMove::SPrime => {
                self.execute_move(CubeMove::ZPrime);
                self.execute_move(CubeMove::F);
                self.execute_move(CubeMove::BPrime);
            }
            CubeMove::S2 => {
                self.execute_move(CubeMove::Z2);
                self.execute_move(CubeMove::F2);
                self.execute_move(CubeMove::B2);
            }
            CubeMove::X => self.rotate_x(),
            CubeMove::XPrime => {
                self.execute_move(CubeMove::X);
                self.execute_move(CubeMove::X);
                self.execute_move(CubeMove::X);
            }
            CubeMove::X2 => {
                self.execute_move(CubeMove::X);
                self.execute_move(CubeMove::X);
            }
            CubeMove::Y => self.rotate_y(),
            CubeMove::YPrime => {
                self.execute_move(CubeMove::Y);
                self.execute_move(CubeMove::Y);
                self.execute_move(CubeMove::Y);
            }
            CubeMove::Y2 => {
                self.execute_move(CubeMove::Y);
                self.execute_move(CubeMove::Y);
            }
            CubeMove::Z => self.rotate_z(),
            CubeMove::ZPrime => {
                self.execute_move(CubeMove::Z);
                self.execute_move(CubeMove::Z);
                self.execute_move(CubeMove::Z);
            }
            CubeMove::Z2 => {
                self.execute_move(CubeMove::Z);
                self.execute_move(CubeMove::Z);
            }
        }
    }
}
