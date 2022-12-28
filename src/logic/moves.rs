use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up = 0,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

