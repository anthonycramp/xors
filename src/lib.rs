pub mod board;

use board::*;

enum Player {
    Player1,
    Player2,
}

struct ScriptedPlayer {
    id: String,
    token: PlayerToken,
    play_list: Vec<BoardLocation>,
    play_index: usize,
}

enum GameError {
    NoMoreMoves,
}

impl ScriptedPlayer {
    fn new(id: &str, token: PlayerToken, play_list: &[BoardLocation]) -> Self {
        Self {
            id: String::from(id),
            token: token,
            play_list: play_list.into(),
            play_index: 0,
        }
    }
    fn play(&mut self) -> Result<(PlayerToken, BoardLocation), GameError> {
        if self.play_index == self.play_list.len() {
            Err(GameError::NoMoreMoves)
        } else {
            let ret = (self.token.clone(), self.play_list[self.play_index].clone());
            self.play_index += 1;
            Ok(ret)
        }
    }
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Player1,
    Player2,
    Tie,
}

struct Game {
    player1: Option<ScriptedPlayer>,
    player2: Option<ScriptedPlayer>,
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

    fn register_player(&mut self, player: ScriptedPlayer) {
        match self.player1 {
            None => self.player1 = Some(player),
            _ => self.player2 = Some(player),
        }
    }

    fn play(&mut self) -> GameResult {
        loop {
            self.player_move();
            if let Some(res) = self.result() {
                return res;
            }
        }
    }

    fn player_move(&mut self) {
        match self.next_play {
            Player::Player1 => {
                if let Ok((token, location)) = self.player1.as_mut().unwrap().play() {
                    self.board.play(location, token);
                    self.next_play = Player::Player2;
                } else {
                    panic!("Player1 out of moves");
                }
            }
            Player::Player2 => {
                if let Ok((token, location)) = self.player2.as_mut().unwrap().play() {
                    self.board.play(location, token);
                    self.next_play = Player::Player1;
                } else {
                    panic!("Player2 out of moves")
                }
            }
        }
    }

    fn result(&self) -> Option<GameResult> {
        if self.board.is_top_row_win()
            || self.board.is_left_column_win()
            || self.board.is_left_right_diagonal_win()
        {
            if self.board.get(BoardLocation::TopLeft).unwrap()
                == self.player1.as_ref().unwrap().token
            {
                Some(GameResult::Player1)
            } else {
                Some(GameResult::Player2)
            }
        } else if self.board.is_middle_row_win()
            || self.board.is_centre_column_win()
            || self.board.is_right_left_diagonal_win()
        {
            if self.board.get(BoardLocation::MiddleCentre).unwrap()
                == self.player1.as_ref().unwrap().token
            {
                Some(GameResult::Player1)
            } else {
                Some(GameResult::Player2)
            }
        } else if self.board.is_bottom_row_win() || self.board.is_right_column_win() {
            if self.board.get(BoardLocation::BottomRight).unwrap()
                == self.player1.as_ref().unwrap().token
            {
                Some(GameResult::Player1)
            } else {
                Some(GameResult::Player2)
            }
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
        let player1 = ScriptedPlayer::new(
            "Yasmin",
            PlayerToken::Cross,
            &vec![
                BoardLocation::MiddleCentre,
                BoardLocation::TopCentre,
                BoardLocation::BottomCentre,
            ],
        );
        let player2 = ScriptedPlayer::new(
            "Mummy",
            PlayerToken::Cross,
            &vec![BoardLocation::TopLeft, BoardLocation::MiddleLeft],
        );

        game.register_player(player1);
        game.register_player(player2);
        let result = game.play();
        assert_eq!(Some(GameResult::Player1), game.result());
    }
}
