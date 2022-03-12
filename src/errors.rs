use std::fmt;

use crate::Direction;

// Use a structure to declare custom 'Falling off the hypercube' error.
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

// struct for custom loop error.
#[derive(Debug)]
pub struct LoopError {
    pub side: LoopSide,
    pub count: usize,
}

// The loop side that is missing.
#[derive(Debug)]
pub enum LoopSide {
    Beginning,
    Ending,
}

// Implementing std::fmt::Display for LoopError
impl fmt::Display for LoopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.side {
            LoopSide::Beginning => {
                write!(
                    f,
                    "ERROR: The loop ending at instruction '{}' has no beginning.",
                    self.count
                )
            }
            LoopSide::Ending => {
                write!(
                    f,
                    "ERROR: The loop starting at instruction '{}' has no ending.",
                    self.count
                )
            }
        }
    }
}
