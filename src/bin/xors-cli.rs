use xors::board::{BoardLocation, GameBoard, PlayerToken};

fn main() {
    let mut board = GameBoard::default();
    board.play(BoardLocation::TopCentre, PlayerToken::Cross);
    board.play(BoardLocation::MiddleLeft, PlayerToken::Nought);
    println!("{:?}", &board);
}
