
use crate::frames::Frame;

pub struct Game<'a> {
    pub turn: usize,
    pub bowlers: Vec<Bowler<'a>>,
}

impl<'a> Game<'a> {

    /// Performs a bowl in a game.
    /// 
    /// The "next bowl" is the first incomplete frame. Return value
    /// is success, which will be true unless there is some kind of
    /// memory error, or if the game is already completed.
    pub fn bowl(&'a mut self, score: u8) -> bool {

        let bowler = &self.bowlers[self.turn];

        let mut frame: Option<&Frame> = None;
        for each in bowler.frames.iter() {
            if !each.is_bowled() {
                frame = Some(each);
                break;
            }
        }
        if frame.is_none() {
            return false;
        }

        let frame = frame.unwrap();
        let bowled = frame.is_bowled();

        let bowler = &mut self.bowlers[self.turn];
        let success = bowler.bowl(score);

        if success && bowled {
            self.turn += 1;
        }

        success
    }
}

/// A bowler is someone who bowls.
pub struct Bowler<'a> {
    pub frames: [Frame<'a>; 10],

    pub bowls: [Option<u8>; 21],
}

impl<'a> Bowler<'a> {

    /// Resets/initializes a bowler
    pub fn init(&'a mut self) {
        *self = Bowler {
            frames: [Frame { pins: [&None; 3] }; 10],

            bowls: [None; 21],
        };

        for (i, frame) in self.frames.iter_mut().enumerate() {
            frame.pins[0] = &self.bowls[i*2];
            frame.pins[1] = &self.bowls[i*2+1];
            frame.pins[2] = &None;
        }
        self.frames[9].pins[2] = &self.bowls[2];
    }

    /// A mutating function which performs a player's next bowl.
    /// 
    /// The "next bowl" is the first incomplete frame. Return value
    /// is success, which will be true unless there is some kind of
    /// memory error, or if the game is already completed.
    pub fn bowl(&'a mut self, score: u8) -> bool {

        let mut frame_index: usize = 0;
        let mut frame: Option<&Frame> = None;
        for (i, each) in self.frames.iter().enumerate() {
            if !each.is_bowled() {
                frame = Some(each);
                frame_index = i;
                break;
            }
        }
        if frame.is_none() {
            return false
        }
        let frame_index = frame_index;
        let frame = frame.unwrap();

        let mut pins = [*frame.pins[0], *frame.pins[1]];

        match (pins[0], pins[1]) {
            (None, None) => pins[0] = Some(score),
            (Some(_), None) => pins[1] = Some(score),
            _ => {},
        }

        self.edit(frame_index, pins);

        true
    }

    /// Edits a frame of the bowler.
    /// 
    /// `frame_index` represents the index of the frame, and `pins`
    /// should be what the frame's pins are to be set to. Strikes/spares/etc
    /// will be automatically calculated.
    pub fn edit(&'a mut self, frame_index: usize, pins: [Option<u8>; 2]) {
        
        // really stupid workaround so that i can have
        // two mutable borrows on separate array indices.
        let (head, tail) = self.frames.split_at_mut(frame_index);
        let mut frame = &mut tail[0];

        self.bowls[frame_index*2] = pins[0];
        self.bowls[frame_index*2 + 1] = pins[1];

        // update previous frame's bonus pins if needed
        if frame_index > 0 {
            let last_frame = &mut head[head.len()-1];

            if last_frame.is_strike() {
                if frame.is_strike() {
                    last_frame.pins[2] = &self.bowls[(frame_index+1)*2];
                } else {
                    last_frame.pins[2] = &self.bowls[(frame_index)*2 + 1];
                }
            }
        }

        // update bonus pin references for current frame
        if frame.is_strike() {
            frame.pins[1] = &self.bowls[frame_index*2 + 2];
            frame.pins[2] = &self.bowls[frame_index*2 + 3];
        } else if frame.is_spare() {
            frame.pins[2] = &self.bowls[(frame_index+1)*2];
        } else {
            frame.pins[2] = &None;
        }
    }
}
