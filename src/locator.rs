use crate::errors::MovError;

// The location struct to hold the current position of our program
#[derive(Clone, Copy)]
pub struct Loc {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub w: usize,
    count: usize,
}

// The directions in which we can move in our 'hypercube' of memory
#[derive(Debug)]
pub enum Direction {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
    WPos,
    WNeg,
}

// A macro for the repetitive 'directions' match statement.
macro_rules! direction_pos {
    ($self:ident, $dir:expr, $var:ident, $steps:expr) => {{
        if $self.$var + $steps as usize >= $self.count {
            return Err(MovError { direction: $dir });
        }

        $self.$var += $steps as usize;
    }};
}

macro_rules! direction_neg {
    ($self:ident, $dir:expr, $var:ident, $steps:expr) => {{
        if $self.$var as isize - $steps < 0 {
            return Err(MovError { direction: $dir });
        }

        $self.$var -= $steps as usize;
    }};
}

// A location structure so we can keep track of where we are in memory. This is
// how we keep track while we are interpreting the program.
impl Loc {
    // Create a new Loc structure
    pub fn new(count: usize) -> Self {
        Loc {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
            count,
        }
    }

    // Move a certain direction, a certain number of steps. We prefer using this
    // function over just manually changing the variables of the structure
    // because we need to make sure we are in the bounds of the memory.
    // This also gives us a good way to describe movement.
    pub fn mov(
        &mut self,
        direction: Direction,
        steps: isize,
    ) -> Result<(), MovError> {
        match direction {
            Direction::XPos => {
                direction_pos!(self, Direction::XPos, x, steps)
            }
            Direction::XNeg => {
                direction_neg!(self, Direction::XNeg, x, steps)
            }
            Direction::YPos => {
                direction_pos!(self, Direction::YPos, y, steps)
            }
            Direction::YNeg => {
                direction_neg!(self, Direction::YNeg, y, steps)
            }
            Direction::ZPos => {
                direction_pos!(self, Direction::ZPos, z, steps)
            }
            Direction::ZNeg => {
                direction_neg!(self, Direction::ZNeg, z, steps)
            }
            Direction::WPos => {
                direction_pos!(self, Direction::WPos, w, steps)
            }
            Direction::WNeg => {
                direction_neg!(self, Direction::WNeg, w, steps)
            }
        }

        Ok(())
    }
}
