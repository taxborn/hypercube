use std::fmt;

use crate::Direction;

// Use a unit struct to declare custom 'Falling off the hypercube' error.
pub struct MovError {
    pub direction: Direction,
}

// Implementing std::fmt::Display for MovError
impl fmt::Display for MovError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ERROR: Fell off the hypercube, in the {:?} direction",
            self.direction
        )
    }
}
