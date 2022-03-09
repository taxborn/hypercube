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

impl Loc {
    // Create a new location struct
    pub fn new(count: usize) -> Self {
        Loc {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
            count
        }
    }

    pub fn mov(
        &mut self,
        direction: Direction,
        steps: isize,
    ) -> Result<(), MovError> {
        match direction {
            Direction::XPos => {
                if self.x + steps as usize >= self.count {
                    return Err(MovError {
                        direction: Direction::XPos,
                    });
                }

                self.x += steps as usize;
            }
            Direction::XNeg => {
                if self.x as isize - steps < 0 {
                    return Err(MovError {
                        direction: Direction::XNeg,
                    });
                }

                self.x -= steps as usize;
            }
            Direction::YPos => {
                if self.y + steps as usize >= self.count {
                    return Err(MovError {
                        direction: Direction::YPos,
                    });
                }

                self.y += steps as usize;
            }
            Direction::YNeg => {
                if self.y as isize - steps < 0 {
                    return Err(MovError {
                        direction: Direction::YNeg,
                    });
                }

                self.y -= steps as usize;
            }
            Direction::ZPos => {
                if self.z + steps as usize >= self.count {
                    return Err(MovError {
                        direction: Direction::ZPos,
                    });
                }

                self.z += steps as usize;
            }
            Direction::ZNeg => {
                if self.z as isize - steps < 0 {
                    return Err(MovError {
                        direction: Direction::ZNeg,
                    });
                }

                self.z -= steps as usize;
            }
            Direction::WPos => {
                if self.w + steps as usize >= self.count {
                    return Err(MovError {
                        direction: Direction::WPos,
                    });
                }

                self.w += steps as usize;
            }
            Direction::WNeg => {
                if self.w as isize - steps < 0 {
                    return Err(MovError {
                        direction: Direction::WNeg,
                    });
                }

                self.w -= steps as usize;
            }
        }

        Ok(())
    }
}
