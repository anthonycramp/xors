#[derive(Debug, PartialEq, Clone)]
pub enum PlayerToken {
    Nought,
    Cross,
}

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
    top_left: Option<PlayerToken>,
    top_centre: Option<PlayerToken>,
    top_right: Option<PlayerToken>,
    middle_left: Option<PlayerToken>,
    middle_centre: Option<PlayerToken>,
    middle_right: Option<PlayerToken>,
    bottom_left: Option<PlayerToken>,
    bottom_centre: Option<PlayerToken>,
    bottom_right: Option<PlayerToken>,
}

impl GameBoard {
    pub fn play(&mut self, location: BoardLocation, player: PlayerToken) {
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

enum Player {
    Player1,
    Player2,
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Player1,
    Player2,
    Tie,
}

struct Game {
    player1: Option<PlayerToken>,
    player2: Option<PlayerToken>,
    next_play: Player,
    board: GameBoard,
}

impl Game {
    fn new() -> Self {
        Game {
            player1: None,
            player2: None,
            next_play: Player::Player1,
            board: GameBoard::default(),
        }
    }

    fn register_player(&mut self) {
        match self.player1 {
            None => self.player1 = Some(PlayerToken::Cross),
            Some(PlayerToken::Cross) => self.player2 = Some(PlayerToken::Nought),
            _ => self.player2 = Some(PlayerToken::Cross),
        }
    }

    fn play(&mut self, location: BoardLocation) {
        match self.next_play {
            Player::Player1 => {
                self.board.play(location, self.player1.clone().unwrap());
                self.next_play = Player::Player2;
            }
            Player::Player2 => {
                self.board.play(location, self.player2.clone().unwrap());
                self.next_play = Player::Player1;
            }
        }
    }

    fn result(&self) -> Option<GameResult> {
        if (self.board.is_top_row_win()
            || self.board.is_left_column_win()
            || self.board.is_left_right_diagonal_win())
        {
            None
        } else if (self.board.is_middle_row_win()
            || self.board.is_centre_column_win()
            || self.board.is_right_left_diagonal_win())
        {
            None
        } else if (self.board.is_bottom_row_win() || self.board.is_right_column_win()) {
            None
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_top_row_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_top_row_win());
        board.play(BoardLocation::TopCentre, PlayerToken::Cross);
        assert!(!board.is_top_row_win());
        board.play(BoardLocation::TopRight, PlayerToken::Cross);
        assert!(!board.is_top_row_win());
        board.play(BoardLocation::TopLeft, PlayerToken::Cross);
        assert!(board.is_top_row_win());
    }

    #[test]
    pub fn test_is_middle_row_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_middle_row_win());
        board.play(BoardLocation::MiddleCentre, PlayerToken::Cross);
        assert!(!board.is_middle_row_win());
        board.play(BoardLocation::MiddleRight, PlayerToken::Cross);
        assert!(!board.is_middle_row_win());
        board.play(BoardLocation::MiddleLeft, PlayerToken::Cross);
        assert!(board.is_middle_row_win());
    }

    #[test]
    pub fn test_is_bottom_row_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_bottom_row_win());
        board.play(BoardLocation::BottomCentre, PlayerToken::Cross);
        assert!(!board.is_bottom_row_win());
        board.play(BoardLocation::BottomRight, PlayerToken::Cross);
        assert!(!board.is_bottom_row_win());
        board.play(BoardLocation::BottomLeft, PlayerToken::Cross);
        assert!(board.is_bottom_row_win());
    }

    #[test]
    pub fn test_is_left_column_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_left_column_win());
        board.play(BoardLocation::TopLeft, PlayerToken::Cross);
        assert!(!board.is_left_column_win());
        board.play(BoardLocation::MiddleLeft, PlayerToken::Cross);
        assert!(!board.is_left_column_win());
        board.play(BoardLocation::BottomLeft, PlayerToken::Cross);
        assert!(board.is_left_column_win());
    }

    #[test]
    pub fn test_is_centre_column_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_centre_column_win());
        board.play(BoardLocation::TopCentre, PlayerToken::Cross);
        assert!(!board.is_centre_column_win());
        board.play(BoardLocation::MiddleCentre, PlayerToken::Cross);
        assert!(!board.is_centre_column_win());
        board.play(BoardLocation::BottomCentre, PlayerToken::Cross);
        assert!(board.is_centre_column_win());
    }

    #[test]
    pub fn test_is_right_column_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_right_column_win());
        board.play(BoardLocation::TopRight, PlayerToken::Cross);
        assert!(!board.is_right_column_win());
        board.play(BoardLocation::MiddleRight, PlayerToken::Cross);
        assert!(!board.is_right_column_win());
        board.play(BoardLocation::BottomRight, PlayerToken::Cross);
        assert!(board.is_right_column_win());
    }

    #[test]
    pub fn test_is_left_right_diagonal_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_left_right_diagonal_win());
        board.play(BoardLocation::TopLeft, PlayerToken::Cross);
        assert!(!board.is_left_right_diagonal_win());
        board.play(BoardLocation::MiddleCentre, PlayerToken::Cross);
        assert!(!board.is_left_right_diagonal_win());
        board.play(BoardLocation::BottomRight, PlayerToken::Cross);
        assert!(board.is_left_right_diagonal_win());
    }

    #[test]
    pub fn test_is_right_left_diagonal_win() {
        let mut board = GameBoard::default();
        assert!(!board.is_right_left_diagonal_win());
        board.play(BoardLocation::TopRight, PlayerToken::Cross);
        assert!(!board.is_right_left_diagonal_win());
        board.play(BoardLocation::MiddleCentre, PlayerToken::Cross);
        assert!(!board.is_right_left_diagonal_win());
        board.play(BoardLocation::BottomLeft, PlayerToken::Cross);
        assert!(board.is_right_left_diagonal_win());
    }

    #[test]
    pub fn test_game() {
        let mut game = Game::new();
        game.register_player();
        game.register_player();
        game.play(BoardLocation::MiddleCentre);
        assert_eq!(None, game.result());
        game.play(BoardLocation::TopLeft);
        game.play(BoardLocation::TopCentre);
        game.play(BoardLocation::BottomLeft);
        game.play(BoardLocation::BottomCentre);
        assert_eq!(Some(GameResult::Player1), game.result());
    }
}
