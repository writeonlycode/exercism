// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        use Direction::*;

        match self.d {
            North => Robot { d: East, ..self },
            East => Robot { d: South, ..self },
            South => Robot { d: West, ..self },
            West => Robot { d: North, ..self },
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        use Direction::*;

        match self.d {
            North => Robot { d: West, ..self },
            West => Robot { d: South, ..self },
            South => Robot { d: East, ..self },
            East => Robot { d: North, ..self },
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        use Direction::*;

        match self.d {
            North => Robot { y: self.y + 1, ..self },
            West => Robot { x: self.x - 1, ..self },
            South => Robot { y: self.y - 1, ..self },
            East => Robot { x: self.x + 1, ..self },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut s = Robot { x: self.x, y: self.y, d: self.d };

        for instruction in instructions.chars() {
            match instruction {
                'R' => s = s.turn_right(),
                'L' => s = s.turn_left(),
                'A' => s = s.advance(),
                _ => (),
            };
        }

        Robot { x: s.x, y: s.y, d: s.d }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
