use xors::board::{BoardLocation, BoardToken};
use xors::player::InteractivePlayer;
use xors::InteractiveGame;

fn main() {
    let mut game = InteractiveGame::new();
    let player1 = InteractivePlayer::new("Yasmin", BoardToken::Cross);
    let player2 = InteractivePlayer::new("Mummy", BoardToken::Nought);

    game.register_player(player1);
    game.register_player(player2);
    let _result = game.play();
}
