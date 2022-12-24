#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
}

impl Color {
    pub fn beats(&self, enemy: &Color) -> bool {
        if *self == Color::Red && *enemy == Color::Green {
            return true;
        }
        if *self == Color::Green && *enemy == Color::Yellow {
            return true;
        }
        if *self == Color::Yellow && *enemy == Color::Blue {
            return true;
        }
        if *self == Color::Blue && *enemy == Color::Red {
            return true;
        }
        false
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseColorError;

impl TryFrom<char> for Color {
    type Error = ParseColorError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'r' => Ok(Color::Red),
            'g' => Ok(Color::Green),
            'y' => Ok(Color::Yellow),
            'b' => Ok(Color::Blue),
            _ => Err(ParseColorError),
        }
    }
}
