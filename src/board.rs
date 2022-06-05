#[derive(Debug, PartialEq, Clone)]
pub enum BoardToken {
    Nought,
    Cross,
}

#[derive(Clone)]
pub enum BoardLocation {
    TopLeft,
    TopCentre,
    TopRight,
    MiddleLeft,
    MiddleCentre,
    MiddleRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

#[derive(Default, Debug)]
pub struct GameBoard {
    top_left: Option<BoardToken>,
    top_centre: Option<BoardToken>,
    top_right: Option<BoardToken>,
    middle_left: Option<BoardToken>,
    middle_centre: Option<BoardToken>,
    middle_right: Option<BoardToken>,
    bottom_left: Option<BoardToken>,
    bottom_centre: Option<BoardToken>,
    bottom_right: Option<BoardToken>,
}

impl GameBoard {
    pub fn play(&mut self, location: BoardLocation, player: BoardToken) {
        match location {
            BoardLocation::TopLeft => self.top_left = Some(player),
            BoardLocation::TopCentre => self.top_centre = Some(player),
            BoardLocation::TopRight => self.top_right = Some(player),
            BoardLocation::MiddleLeft => self.middle_left = Some(player),
            BoardLocation::MiddleCentre => self.middle_centre = Some(player),
            BoardLocation::MiddleRight => self.middle_right = Some(player),
            BoardLocation::BottomLeft => self.bottom_left = Some(player),
            BoardLocation::BottomCentre => self.bottom_centre = Some(player),
            BoardLocation::BottomRight => self.bottom_right = Some(player),
        }
    }

    pub fn get(&self, location: BoardLocation) -> Option<BoardToken> {
        match location {
            BoardLocation::TopLeft => self.top_left.clone(),
            BoardLocation::TopCentre => self.top_centre.clone(),
            BoardLocation::TopRight => self.top_right.clone(),
            BoardLocation::MiddleLeft => self.middle_left.clone(),
            BoardLocation::MiddleCentre => self.middle_centre.clone(),
            BoardLocation::MiddleRight => self.middle_right.clone(),
            BoardLocation::BottomLeft => self.bottom_left.clone(),
            BoardLocation::BottomCentre => self.bottom_centre.clone(),
            BoardLocation::BottomRight => self.bottom_right.clone(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.top_left.is_some()
            && self.top_centre.is_some()
            && self.top_right.is_some()
            && self.middle_left.is_some()
            && self.middle_centre.is_some()
            && self.middle_right.is_some()
            && self.bottom_left.is_some()
            && self.bottom_centre.is_some()
            && self.bottom_right.is_some()
    }
    pub fn is_top_row_win(&self) -> bool {
        self.top_left.is_some()
            && self.top_left == self.top_centre
            && self.top_left == self.top_right
    }

    pub fn is_middle_row_win(&self) -> bool {
        self.middle_left.is_some()
            && self.middle_left == self.middle_centre
            && self.middle_left == self.middle_right
    }

    pub fn is_bottom_row_win(&self) -> bool {
        self.bottom_left.is_some()
            && self.bottom_left == self.bottom_centre
            && self.bottom_left == self.bottom_right
    }

    pub fn is_left_column_win(&self) -> bool {
        self.top_left.is_some()
            && self.top_left == self.middle_left
            && self.top_left == self.bottom_left
    }

    pub fn is_centre_column_win(&self) -> bool {
        self.top_centre.is_some()
            && self.top_centre == self.middle_centre
            && self.top_centre == self.bottom_centre
    }

    pub fn is_right_column_win(&self) -> bool {
        self.top_right.is_some()
            && self.top_right == self.middle_right
            && self.top_right == self.bottom_right
    }

    pub fn is_left_right_diagonal_win(&self) -> bool {
        self.top_left.is_some()
            && self.top_left == self.middle_centre
            && self.top_left == self.bottom_right
    }

    pub fn is_right_left_diagonal_win(&self) -> bool {
        self.top_right.is_some()
            && self.top_right == self.middle_centre
            && self.top_right == self.bottom_left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_top_row_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_top_row_win());
        board.play(BoardLocation::TopCentre, BoardToken::Cross);
        assert!(!board.is_top_row_win());
        board.play(BoardLocation::TopRight, BoardToken::Cross);
        assert!(!board.is_top_row_win());
        board.play(BoardLocation::TopLeft, BoardToken::Cross);
        assert!(board.is_top_row_win());
    }

    #[test]
    pub fn test_is_middle_row_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_middle_row_win());
        board.play(BoardLocation::MiddleCentre, BoardToken::Cross);
        assert!(!board.is_middle_row_win());
        board.play(BoardLocation::MiddleRight, BoardToken::Cross);
        assert!(!board.is_middle_row_win());
        board.play(BoardLocation::MiddleLeft, BoardToken::Cross);
        assert!(board.is_middle_row_win());
    }

    #[test]
    pub fn test_is_bottom_row_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_bottom_row_win());
        board.play(BoardLocation::BottomCentre, BoardToken::Cross);
        assert!(!board.is_bottom_row_win());
        board.play(BoardLocation::BottomRight, BoardToken::Cross);
        assert!(!board.is_bottom_row_win());
        board.play(BoardLocation::BottomLeft, BoardToken::Cross);
        assert!(board.is_bottom_row_win());
    }

    #[test]
    pub fn test_is_left_column_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_left_column_win());
        board.play(BoardLocation::TopLeft, BoardToken::Cross);
        assert!(!board.is_left_column_win());
        board.play(BoardLocation::MiddleLeft, BoardToken::Cross);
        assert!(!board.is_left_column_win());
        board.play(BoardLocation::BottomLeft, BoardToken::Cross);
        assert!(board.is_left_column_win());
    }

    #[test]
    pub fn test_is_centre_column_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_centre_column_win());
        board.play(BoardLocation::TopCentre, BoardToken::Cross);
        assert!(!board.is_centre_column_win());
        board.play(BoardLocation::MiddleCentre, BoardToken::Cross);
        assert!(!board.is_centre_column_win());
        board.play(BoardLocation::BottomCentre, BoardToken::Cross);
        assert!(board.is_centre_column_win());
    }

    #[test]
    pub fn test_is_right_column_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_right_column_win());
        board.play(BoardLocation::TopRight, BoardToken::Cross);
        assert!(!board.is_right_column_win());
        board.play(BoardLocation::MiddleRight, BoardToken::Cross);
        assert!(!board.is_right_column_win());
        board.play(BoardLocation::BottomRight, BoardToken::Cross);
        assert!(board.is_right_column_win());
    }

    #[test]
    pub fn test_is_left_right_diagonal_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_left_right_diagonal_win());
        board.play(BoardLocation::TopLeft, BoardToken::Cross);
        assert!(!board.is_left_right_diagonal_win());
        board.play(BoardLocation::MiddleCentre, BoardToken::Cross);
        assert!(!board.is_left_right_diagonal_win());
        board.play(BoardLocation::BottomRight, BoardToken::Cross);
        assert!(board.is_left_right_diagonal_win());
    }

    #[test]
    pub fn test_is_right_left_diagonal_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_right_left_diagonal_win());
        board.play(BoardLocation::TopRight, BoardToken::Cross);
        assert!(!board.is_right_left_diagonal_win());
        board.play(BoardLocation::MiddleCentre, BoardToken::Cross);
        assert!(!board.is_right_left_diagonal_win());
        board.play(BoardLocation::BottomLeft, BoardToken::Cross);
        assert!(board.is_right_left_diagonal_win());
    }
}
