use crate::board::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

pub struct ScriptedPlayer {
    pub id: String,
    pub token: BoardToken,
    play_list: Vec<BoardLocation>,
    play_index: usize,
}

pub struct InteractivePlayer {
    pub id: String,
    pub token: BoardToken,
}

pub enum PlayerError {
    NoMoreMoves,
    InvalidLocation,
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

    pub fn new_random(id: &str, token: BoardToken) -> Self {
        let mut vec: Vec<u32> = (1..=9).collect();
        vec.shuffle(&mut thread_rng());
        let play_list: Vec<BoardLocation> = vec
            .iter()
            .map(|i| match i {
                1 => BoardLocation::TopLeft,
                2 => BoardLocation::TopCentre,
                3 => BoardLocation::TopRight,
                4 => BoardLocation::MiddleLeft,
                5 => BoardLocation::MiddleCentre,
                6 => BoardLocation::MiddleRight,
                7 => BoardLocation::BottomLeft,
                8 => BoardLocation::BottomCentre,
                9 => BoardLocation::BottomRight,
                _ => unreachable!("vec above is defined with content 1 .. 9"),
            })
            .collect();
        Self::new(id, token, &play_list)
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

impl InteractivePlayer {
    pub fn new(id: &str, token: BoardToken) -> Self {
        Self {
            id: String::from(id),
            token,
        }
    }

    pub fn play(&self) -> Result<(BoardToken, BoardLocation), PlayerError> {
        print!("Enter a location (1-9): ");
        io::stdout().flush().expect("Error writing to screen");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return Err(PlayerError::InvalidLocation);
        }
        let input = input.trim();
        if let Ok(board_location) = input.parse::<u32>() {
            match board_location {
                1 => Ok((self.token.clone(), BoardLocation::TopLeft)),
                2 => Ok((self.token.clone(), BoardLocation::TopCentre)),
                3 => Ok((self.token.clone(), BoardLocation::TopRight)),
                4 => Ok((self.token.clone(), BoardLocation::MiddleLeft)),
                5 => Ok((self.token.clone(), BoardLocation::MiddleCentre)),
                6 => Ok((self.token.clone(), BoardLocation::MiddleRight)),
                7 => Ok((self.token.clone(), BoardLocation::BottomLeft)),
                8 => Ok((self.token.clone(), BoardLocation::BottomCentre)),
                9 => Ok((self.token.clone(), BoardLocation::BottomRight)),
                _ => Err(PlayerError::InvalidLocation),
            }
        } else {
            Err(PlayerError::InvalidLocation)
        }
    }
}
