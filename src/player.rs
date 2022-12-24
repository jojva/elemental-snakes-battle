#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    Top,
    Bottom,
}

impl Player {
    pub fn next(&self) -> Self {
        match self {
            Player::Top => Player::Bottom,
            Player::Bottom => Player::Top,
        }
    }
}
