/// This represents a frame on a scorecard.
pub struct Frame<'a> {
    pub bowls: [Option<u8>; 2],
    pub bonus: [Option<&'a u8>; 2],
}

impl Frame<'_> {
    /// Returns true if both bowls for this frame have been bowled, and not all pins were knocked down.
    pub fn is_open(&self) -> bool {
        if let (Some(b1), Some(b2)) = (self.bowls[0], self.bowls[1]) {
            b1 + b2 != 10
        } else {
            false
        }
    }

    /// Returns true if the first bowl knocked down 10 pins.
    pub fn is_strike(&self) -> bool {
        if let Some(b) = self.bowls[0] {
            b == 10
        } else {
            false
        }
    }

    pub fn is_spare(&self) -> bool {
        if let (Some(b1), Some(b2)) = (self.bowls[0], self.bowls[1]) {
            b1 + b2 == 10 && b2 != 0
        } else {
            false
        }
    }

    pub fn is_complete(&self) -> bool {
        if let Some(b1) = self.bowls[0] {
            if b1 == 10 {
                self.bonus[0].is_some() && self.bonus[1].is_some()
            } else if let Some(b2) = self.bowls[1] {
                if b1 + b2 == 10 {
                    self.bonus[0].is_some()
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Returns the number of points that this frame has scored so far.
    /// Should not be displayed on a scorecard until [`is_complete()`] returns true.
    pub fn get_points(&self) -> u8 {
        let mut points = self.bowls[0].unwrap_or(0) + self.bowls[1].unwrap_or(0);
        if let Some(a) = self.bonus[0] {
            points += *a;
        }
        if let Some(a) = self.bonus[1] {
            points += *a;
        }
        points
    }
}
