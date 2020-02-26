

/// This represents a frame on a scorecard.
pub struct Frame<'a> {
    /// Represents a reference to a bowl, where a bowl may or may not exist.
    /// The order in which they are referred to is [[bowls], [bonus]].
    pub pins: [&'a Option<u8>; 3],
}

impl<'a> Copy for Frame<'a> {}

impl Clone for Frame<'_> {
    fn clone(&self) -> Self {
        Frame {
            pins: self.pins,
        }
    }
}

impl Frame<'_> {

    /// Returns true if both bowls for this frame have been bowled, and not all pins were knocked down.
    pub fn is_open(&self) -> bool {
        if let (Some(b1), Some(b2)) = (self.pins[0], self.pins[1]) {
            *b1 + *b2 != 10
        } else {
            false
        }
    }

    /// Returns true if the first bowl knocked down 10 pins.
    pub fn is_strike(&self) -> bool {
        if let Some(b) = self.pins[0] {
            *b == 10
        } else {
            false
        }
    }

    pub fn is_spare(&self) -> bool {
        if let (Some(b1), Some(b2)) = (self.pins[0], self.pins[1]) {
            *b1 + *b2 == 10 && *b2 != 0
        } else {
            false
        }
    }

    pub fn is_bowled(&self) -> bool {
        match (self.pins[0], self.pins[1]) {
            (Some(_), Some(_)) => true,
            (Some(b1), None) => *b1 == 10,
            _ => false,
        }
    }

    pub fn display_points(&self) -> bool {
        match (self.pins[0], self.pins[1], self.pins[2]) {
            (Some(_), Some(_), Some(_)) => true,
            (Some(b1), Some(b2), None) => *b1 + *b2 != 10,
            _ => false,
        }
    }

    /// Returns the number of points that this frame has scored so far.
    /// Should not be displayed on a scorecard until [`is_complete()`] returns true.
    pub fn get_points(&self) -> u8 {
        self.pins[0].unwrap_or(0)
                + self.pins[1].unwrap_or(0)
                + self.pins[2].unwrap_or(0)
    }
}
