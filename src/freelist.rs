use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct SortedVector {
    data: Vec<FreeSpace>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct FreeSpace {
    cursor: usize,
    space: usize,
}

/// Contains cursor and space of empty spaces, to be used by VACUUM directive and insert
pub struct FreeList {
    data: Vec<FreeSpace>,
}


impl FreeList {
    pub fn new() -> FreeList {
        FreeList {
            // todo(): inefficient struct, after insert/remove needs to shift elements
            // todo(): implement locks for concurrent access
            data: Vec::new(),
        }
    }

    pub fn insert_free_space(&mut self, cursor: usize, space: usize) {
        let value = FreeSpace { cursor, space };
        let pos = match self.data.binary_search(&value) {
            Ok(pos) | Err(pos) => pos,
        };

        self.data.insert(pos, value);
    }

    pub fn retrieve_free_space(&mut self, space: usize) -> Option<usize> {
        let space_cursor = FreeSpace{space: space, cursor: 0};

        if let Some(val) = self.retrieve_equal_or_bigger_than(&space_cursor) {
            return Some(val.cursor)
        }

        return None
    }

    pub fn compact() {

    }

    fn retrieve_equal_or_bigger_than(&mut self, value: &FreeSpace) -> Option<FreeSpace> {
        let mut ret;

        // search for the first item in the list that have equal or bigger space available
        let pos = match self.data.binary_search(value) {
            Ok(pos) => pos,
            Err(pos) if pos < self.data.len() => pos,
            _ => return None,
        };

        ret = self.data.remove(pos);

        // store again the free space if the space claimed has been bigger than the space
        // that is going to be filled
        if ret.space >= value.space {
            let free_space = FreeSpace {
                space: ret.space - value.space,
                cursor: ret.cursor + value.space,
            };

            self.data.insert(pos, free_space);
        }

        Some(ret)
    }
}

impl Ord for FreeSpace {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut order = self.space.cmp(&other.space);
        if order == Ordering::Equal {
            return self.cursor.cmp(&other.cursor)
        }

        return order
    }
}
