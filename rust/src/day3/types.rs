use std::{cmp::Ordering, collections::BTreeMap};

pub(super) type SchematicType = BTreeMap<Pos, SchematicData>;

#[derive(Debug, Eq, PartialEq)]
pub(super) enum SchematicData {
    Dot,
    Symbol(char),
    Digit(u32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(super) struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn added_pos(&self, dx: i32, dy: i32) -> Self {
        Pos::new(self.x + dx, self.y + dy)
    }

    pub fn left(&self) -> Self {
        self.added_pos(1, 0)
    }

    pub fn right(&self) -> Self {
        self.added_pos(-1, 0)
    }
}

impl From<(i32, i32)> for Pos {
    fn from(arg: (i32, i32)) -> Self {
        let (x, y) = arg;
        Pos::new(x, y)
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.x, self.y) == (other.x, other.y) {
            Ordering::Equal
        } else if self.y == other.y {
            if self.x > other.x {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            if self.y > other.y {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
