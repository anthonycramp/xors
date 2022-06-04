#[derive(Debug, PartialEq)]
enum PlayerToken {
    Nought,
    Cross,
}

enum BoardLocation {
    TopLeft,
    TopCentre,
    TopRight,
    MiddleLeft,
    MiddleCentre,
    MiddleRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

#[derive(Default, Debug)]
struct GameBoard {
    top_left: Option<PlayerToken>,
    top_centre: Option<PlayerToken>,
    top_right: Option<PlayerToken>,
    middle_left: Option<PlayerToken>,
    middle_centre: Option<PlayerToken>,
    middle_right: Option<PlayerToken>,
    bottom_left: Option<PlayerToken>,
    bottom_centre: Option<PlayerToken>,
    bottom_right: Option<PlayerToken>,
}

impl GameBoard {
    fn play(&mut self, location: BoardLocation, player: PlayerToken) {
        match location {
            BoardLocation::TopLeft => self.top_left = Some(player),
            BoardLocation::TopCentre => self.top_centre = Some(player),
            BoardLocation::TopRight => self.top_right = Some(player),
            BoardLocation::MiddleLeft => self.middle_left = Some(player),
            BoardLocation::MiddleCentre => self.middle_centre = Some(player),
            BoardLocation::MiddleRight => self.middle_right = Some(player),
            BoardLocation::BottomLeft => self.bottom_left = Some(player),
            BoardLocation::BottomCentre => self.bottom_centre = Some(player),
            BoardLocation::BottomRight => self.bottom_right = Some(player),
        }
    }

    fn check_for_win_in_top_row(&self) -> bool {
        self.top_left.is_some()
            && self.top_left == self.top_centre
            && self.top_left == self.top_right
    }
}
