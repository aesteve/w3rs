use crate::blocks::command::Position;
use std::fmt::{Display, Formatter};

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x={},y={}}}", self.x, self.y)
    }
}
