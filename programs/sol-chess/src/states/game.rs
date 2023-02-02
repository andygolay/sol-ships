use crate::*;

pub const SEED_GAME: &[u8] = b"game";

#[account]
pub struct Game {
    pub board: Board,
    pub state: GameState,
    pub white: Option<Pubkey>,
    pub black: Option<Pubkey>,
    pub enpassant: Option<Square>,
}

impl Game {
    pub fn pda(payer: Pubkey, game_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[SEED_GAME, payer.as_ref(), &game_id.to_be_bytes()],
            &crate::ID,
        )
    }

    pub fn is_valid_move(&self, color: Color, from: Square, to: Square) -> bool {
        let valid_moves = match self.board.get_piece(from) {
            Piece::WhitePawn | Piece::BlackPawn => self.get_valid_pawn_moves(color, from),
            Piece::WhiteRook | Piece::BlackRook => self.get_valid_rook_moves(color, from),
            Piece::WhiteKnight | Piece::BlackKnight => self.get_valid_knight_moves(color, from),
            Piece::WhiteBishop | Piece::BlackBishop => self.get_valid_bishop_moves(color, from),
            Piece::WhiteQueen | Piece::BlackQueen => self.get_valid_queen_moves(color, from),
            _ => vec![],
        };

        if valid_moves.contains(&to) {
            return true;
        }
        return false;
    }

    pub fn get_valid_pawn_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        // pawn steps
        let forward_square = square.get_square_forward(color);
        if self.board.get_piece(forward_square).is_empty() {
            // one step
            valid_squares.push(forward_square);

            let double_forward_square = square.get_square_double_forward(color);
            // double step
            if self.board.get_piece(double_forward_square).is_empty()
                && square.is_starting_pawn_square(color)
            {
                valid_squares.push(double_forward_square);
            }
        }

        // pawn eats forward left (piece or enpassant)
        if !square.is_leftmost_file_square_relative(color) {
            let left_forward_square = square.get_square_forward_left(color);
            let left_forward_piece = self.board.get_piece(left_forward_square);

            if (left_forward_piece.is_not_empty()
                && left_forward_piece.get_color().is_opposite(color))
                || (self.enpassant.is_some() && left_forward_square == self.enpassant.unwrap())
            {
                valid_squares.push(left_forward_square);
            }
        }

        // pawn eats forward right (piece or enpassant)
        if !square.is_rightmost_file_square_relative(color) {
            let right_forward_square = square.get_square_forward_right(color);
            let right_forward_piece = self.board.get_piece(right_forward_square);

            if (right_forward_piece.is_not_empty()
                && right_forward_piece.get_color().is_opposite(color))
                || (self.enpassant.is_some() && right_forward_square == self.enpassant.unwrap())
            {
                valid_squares.push(right_forward_square);
            }
        }

        return valid_squares;
    }

    pub fn get_valid_rook_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        // move
        valid_squares.extend(self.board.get_upper_squares_empty(square));
        valid_squares.extend(self.board.get_lower_squares_empty(square));
        valid_squares.extend(self.board.get_right_squares_empty(square));
        valid_squares.extend(self.board.get_left_squares_empty(square));

        // eat
        let upper_piece = self.board.get_upper_piece(square);
        if upper_piece.is_some() && upper_piece.unwrap().0.get_color().is_opposite(color) {
            valid_squares.push(upper_piece.unwrap().1)
        }

        let lower_piece = self.board.get_lower_piece(square);
        if lower_piece.is_some() && lower_piece.unwrap().0.get_color().is_opposite(color) {
            valid_squares.push(lower_piece.unwrap().1)
        }

        let right_piece = self.board.get_right_piece(square);
        if right_piece.is_some() && right_piece.unwrap().0.get_color().is_opposite(color) {
            valid_squares.push(right_piece.unwrap().1)
        }

        let left_piece = self.board.get_left_piece(square);
        if left_piece.is_some() && left_piece.unwrap().0.get_color().is_opposite(color) {
            valid_squares.push(left_piece.unwrap().1)
        }

        return valid_squares;
    }

    pub fn get_valid_knight_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        for jump in square.get_knight_jumps() {
            let piece = self.board.get_piece(jump);
            if piece.is_empty() || piece.get_color().is_opposite(color) {
                valid_squares.push(jump);
            }
        }

        return valid_squares;
    }

    pub fn get_valid_bishop_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        // move
        valid_squares.extend(self.board.get_upper_right_squares_empty(square));
        valid_squares.extend(self.board.get_lower_right_squares_empty(square));
        valid_squares.extend(self.board.get_upper_left_squares_empty(square));
        valid_squares.extend(self.board.get_lower_left_squares_empty(square));

        // eat
        let upper_right_piece = self.board.get_upper_right_piece(square);
        if upper_right_piece.is_some()
            && upper_right_piece.unwrap().0.get_color().is_opposite(color)
        {
            valid_squares.push(upper_right_piece.unwrap().1)
        }

        let upper_left_piece = self.board.get_upper_left_piece(square);
        if upper_left_piece.is_some() && upper_left_piece.unwrap().0.get_color().is_opposite(color)
        {
            valid_squares.push(upper_left_piece.unwrap().1)
        }

        let lower_right_piece = self.board.get_lower_right_piece(square);
        if lower_right_piece.is_some()
            && lower_right_piece.unwrap().0.get_color().is_opposite(color)
        {
            valid_squares.push(lower_right_piece.unwrap().1)
        }

        let lower_left_piece = self.board.get_lower_left_piece(square);
        if lower_left_piece.is_some() && lower_left_piece.unwrap().0.get_color().is_opposite(color)
        {
            valid_squares.push(lower_left_piece.unwrap().1)
        }

        return valid_squares;
    }

    pub fn get_valid_queen_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        valid_squares.extend(self.get_valid_rook_moves(color, square));
        valid_squares.extend(self.get_valid_bishop_moves(color, square));

        return valid_squares;
    }

    pub fn move_piece(&mut self, color: Color, from: Square, to: Square) {
        let current_enpassant = self.enpassant.clone();
        self.reset_enpassant();

        match self.board.get_piece(from) {
            Piece::WhitePawn | Piece::BlackPawn => {
                // passant eat
                if self.enpassant.is_some() {}
                if current_enpassant.is_some() && to == current_enpassant.unwrap() {
                    self.board.eat_piece(to.get_square_backward(color))
                }
                // double step
                else if to.is_double_forward(color, from) {
                    self.set_enpassant(from.get_square_forward(color));
                }
            }
            _ => {}
        };

        self.board.move_piece(from, to);
    }

    pub fn get_current_player_pubkey(&self) -> Pubkey {
        if self.state.get_current_player_turn().is_white() {
            self.white.unwrap()
        } else {
            self.black.unwrap()
        }
    }

    pub fn get_current_player_color(&self) -> Color {
        if self.state.get_current_player_turn().is_white() {
            return Color::White;
        } else {
            return Color::Black;
        }
    }

    pub fn join_game(&mut self, user: Pubkey, color: Color) {
        if color.is_white() {
            self.white = Some(user);
        } else {
            self.black = Some(user);
        }
    }

    pub fn color_available(&self, color: Color) -> bool {
        if color.is_white() {
            self.white.is_none()
        } else {
            self.black.is_none()
        }
    }

    pub fn is_full(&self) -> bool {
        self.white.is_some() && self.black.is_some()
    }

    pub fn start_game(&mut self) {
        self.state = GameState::White;
    }

    pub fn next_turn(&mut self) {
        self.state = self.state.next_turn()
    }

    pub fn set_enpassant(&mut self, square: Square) {
        self.enpassant = Some(square);
    }

    pub fn reset_enpassant(&mut self) {
        self.enpassant = None;
    }
}

pub trait GameAccount {
    fn new(&mut self) -> Result<()>;
}

impl GameAccount for Account<'_, Game> {
    fn new(&mut self) -> Result<()> {
        self.board = Board::default();
        self.state = GameState::Waiting;
        self.white = None;
        self.black = None;
        self.enpassant = None;
        Ok(())
    }
}
