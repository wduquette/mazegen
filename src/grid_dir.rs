use std::fmt;
use std::str::FromStr;

/// The directions between cells in this grid.
/// TODO: Should be an associated type?
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GridDirection {
    North,
    South,
    East,
    West,
}

impl fmt::Display for GridDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GridDirection::North => write!(f, "north"),
            GridDirection::South => write!(f, "south"),
            GridDirection::East => write!(f, "east"),
            GridDirection::West => write!(f, "west"),
        }
    }
}

impl FromStr for GridDirection {
    type Err = String;

    fn from_str(dir: &str) -> Result<Self, Self::Err> {
        match dir {
            "north" => Ok(GridDirection::North),
            "south" => Ok(GridDirection::South),
            "east" => Ok(GridDirection::East),
            "west" => Ok(GridDirection::West),
            _ => Err(format!("expected direction, got \"{}\"", dir)),
        }
    }
}


// TODO: Tests for formatting.
