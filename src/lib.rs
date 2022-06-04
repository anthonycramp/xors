pub mod board;

use board::*;

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
