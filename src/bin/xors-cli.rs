fn main() {
    let mut board = GameBoard::default();
    board.play(BoardLocation::TopCentre, PlayerToken::Cross);
    board.play(BoardLocation::MiddleLeft, PlayerToken::Nought);
    println!("{:?}", &board);
}
