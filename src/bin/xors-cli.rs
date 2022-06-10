use xors::board::{BoardLocation, BoardToken};
use xors::player::ScriptedPlayer;
use xors::Game;

fn main() {
    let mut game = Game::new();
    let player1 = ScriptedPlayer::new(
        "Yasmin",
        BoardToken::Cross,
        &[
            BoardLocation::MiddleCentre,
            BoardLocation::TopCentre,
            BoardLocation::BottomCentre,
        ],
    );
    let player2 = ScriptedPlayer::new(
        "Mummy",
        BoardToken::Nought,
        &[BoardLocation::TopLeft, BoardLocation::MiddleLeft],
    );

    game.register_player(player1);
    game.register_player(player2);
    let _result = game.play();
}
