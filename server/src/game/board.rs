// ( 3 * 3 ) * ( 3 * 3 ) => 81
pub const BOARD_SIZE: usize = 81;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FieldState {
    Free,
    Cross,
    Nought, // Zero
}

pub struct Board {
    fields: [FieldState; BOARD_SIZE],
    playable_boards: [FieldState ; 9],
}

impl Board {
    pub fn new() -> Self {
        Self { 
            fields: [FieldState::Free; BOARD_SIZE],
            playable_boards: [FieldState::Free; 9],
        }
    }

    pub fn is_valid(&self, loc: i32) -> bool {
        println!("{loc}");
        !(loc < 0 || loc >= self.fields.len() as i32)
    }

    pub fn is_inner_board_playable(&self, z: usize) -> bool {
        self.playable_boards[z] == FieldState::Free
    }

    pub fn set_inner_board_result(&mut self, z: usize, state: FieldState) {
        self.playable_boards[z] = state;
    }

    fn check_board_win(offset: usize, state: FieldState, fields: &[FieldState]) -> bool {
        if offset + 9 > fields.len() {
            return false;
        }

        let mut d = [0i32; 2];

        for i in 0..3 {
            if fields[offset + 3 * i + i]       == state { d[0] += 1; }
            if fields[offset + 3 * (2 - i) + i] == state { d[1] += 1; }

            let mut hv = [0i32; 2];

            for j in 0..3 {
                if fields[offset + 3 * i + j] == state { hv[0] += 1; }
                if fields[offset + 3 * j + i] == state { hv[1] += 1; }
            }

            if hv[0] == 3 || hv[1] == 3 { return true; }
        }

        d[0] == 3 || d[1] == 3
    }
    
    pub fn check_game_win(&self, state: FieldState) -> bool {
        println!("{:?}", self.playable_boards);
        Self::check_board_win(0, state, &self.playable_boards)
    }

    pub fn check_inner_board_wins(&self, board: usize, state: FieldState) -> bool {
        if self.playable_boards[board] != FieldState::Free {
            return false;
        }

        Self::check_board_win(9 * board, state, &self.fields)
    }
}

impl std::ops::Index<usize> for Board {
    type Output = FieldState;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

impl std::ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.fields[index]
    }
}
