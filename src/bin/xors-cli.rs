use xors::board::{BoardLocation, BoardToken, GameBoard};

fn main() {
    let mut board = GameBoard::default();
    board.play(BoardLocation::TopCentre, BoardToken::Cross);
    board.play(BoardLocation::MiddleLeft, BoardToken::Nought);
    println!("{:?}", &board);
}
