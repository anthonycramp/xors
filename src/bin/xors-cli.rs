use xors::board::{BoardLocation, BoardToken, GameBoard};
use xors::player::ScriptedPlayer;
use xors::Game;

fn main() {
    let mut game = Game::new();
    let player1 = ScriptedPlayer::new(
        "Yasmin",
        BoardToken::Cross,
        &vec![
            BoardLocation::MiddleCentre,
            BoardLocation::TopCentre,
            BoardLocation::BottomCentre,
        ],
    );
    let player2 = ScriptedPlayer::new(
        "Mummy",
        BoardToken::Nought,
        &vec![BoardLocation::TopLeft, BoardLocation::MiddleLeft],
    );

    game.register_player(player1);
    game.register_player(player2);
    let result = game.play();
}
