use crate::board::*;

pub struct ScriptedPlayer {
    pub id: String,
    pub token: BoardToken,
    play_list: Vec<BoardLocation>,
    play_index: usize,
}

pub enum PlayerError {
    NoMoreMoves,
}

impl ScriptedPlayer {
    pub fn new(id: &str, token: BoardToken, play_list: &[BoardLocation]) -> Self {
        Self {
            id: String::from(id),
            token,
            play_list: play_list.into(),
            play_index: 0,
        }
    }
    pub fn play(&mut self) -> Result<(BoardToken, BoardLocation), PlayerError> {
        if self.play_index == self.play_list.len() {
            Err(PlayerError::NoMoreMoves)
        } else {
            let ret = (self.token.clone(), self.play_list[self.play_index].clone());
            self.play_index += 1;
            Ok(ret)
        }
    }
}
