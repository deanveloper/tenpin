
pub enum Frame<'a> {
    Empty,
    Unfinished(u8),
    Open(u8, u8),
    Spare(u8, &'a u8),
    Strike(&'a u8, &'a u8),
}

impl Frame<'_> {
    pub fn get_bowls(&self) -> (u8, u8) {
        return match self {
            Frame::Empty => (0, 0),
            Frame::Unfinished(bowl) => (*bowl, 0),
            Frame::Open(b1, b2) => (*b1, *b2),
            Frame::Spare(b1, _) => (*b1, 10 - *b1),
            Frame::Strike(_, _) => (10, 0),
        };
    }

    pub fn get_bonus(&self) -> (Option<&u8>, Option<&u8>) {
        return match self {
            Frame::Empty => (Option::None, Option::None),
            Frame::Unfinished(_) => (Option::None, Option::None),
            Frame::Open(_, _) => (Option::None, Option::None),
            Frame::Spare(_, bonus) => (Option::Some(bonus), Option::None),
            Frame::Strike(bon1, bon2) => (Option::Some(bon1), Option::Some(bon2)),
        };
    }

    pub fn get_points(&self) -> u8 {
        let bowls = self.get_bowls();
        let bonus = self.get_bonus();
        let mut points = bowls.0 + bowls.1;
        if let Option::Some(a) = bonus.0 {
            points += *a
        }
        if let Option::Some(a) = bonus.1 {
            points += *a
        }
        points
    }
}
