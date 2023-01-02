use crate::{point::Point, Direction};

pub struct Elf {
    // Where this elf is
    pub location: Point,
    // How this elf wants to move
    pub proposal: Option<Point>,
}

impl Elf {
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            location: Point { x, y },
            proposal: None,
        }
    }
    /// Move this elf to the desired position if it is not none
    pub fn move_location(&mut self) {
        if let Some(new_loc) = self.proposal {
            self.location = new_loc;
        }
    }

    pub fn propose_movement(&self, elf_locations: &[Point], index: usize) -> Option<Point> {
        use crate::Direction::*;
        const DIRECTIONS: [Direction; 4] = [North, South, West, East];

        // Check if is isolated
        if Point::get_adjacent_offsets()
            .iter()
            .all(|p| elf_locations.binary_search(&(*p + self.location)).is_err())
        {
            return None;
        }

        // Check the directions in order, based on the index (round number)
        // Ex: Round1 N S W E
        //     Round2 S W E N
        let direction1 = DIRECTIONS[index % 4];
        let direction2 = DIRECTIONS[(index + 1) % 4];
        let direction3 = DIRECTIONS[(index + 2) % 4];
        let direction4 = DIRECTIONS[(index + 3) % 4];
        let mut direction = None;
        if Point::get_offset_for_direction(direction1)
            .iter()
            .all(|p| elf_locations.binary_search(&(*p + self.location)).is_err())
        {
            direction = Some(direction1);
        } else if Point::get_offset_for_direction(direction2)
            .iter()
            .all(|p| elf_locations.binary_search(&(*p + self.location)).is_err())
        {
            direction = Some(direction2);
        } else if Point::get_offset_for_direction(direction3)
            .iter()
            .all(|p| elf_locations.binary_search(&(*p + self.location)).is_err())
        {
            direction = Some(direction3);
        } else if Point::get_offset_for_direction(direction4)
            .iter()
            .all(|p| elf_locations.binary_search(&(*p + self.location)).is_err())
        {
            direction = Some(direction4);
        }
        match direction {
            Some(North) => Some(self.location + Point::new(0, 1)),
            Some(South) => Some(self.location + Point::new(0, -1)),
            Some(East) => Some(self.location + Point::new(1, 0)),
            Some(West) => Some(self.location + Point::new(-1, 0)),
            None => None,
        }
    }
}
