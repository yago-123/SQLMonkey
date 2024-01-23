use std::cmp::Ordering;

struct SortedVector {
    data: Vec<FreeSpace>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct FreeSpace {
    space: usize,
    cursor: usize,
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
        // cursor is not used during sorting
        // todo(cursor sorting): in the future this will be the paramter for sorting based on
        // todo(cursor sorting): the position of the free space (beginning, middle, end..)
        let space_cursor = FreeSpace{space: space, cursor: 0};

        if let Some(val) = self.retrieve_equal_or_bigger_than(&space_cursor) {
            return Some(val.cursor)
        }

        return None
    }

    pub fn compact() {

    }

    fn retrieve_equal_or_bigger_than(&mut self, expected_amount: &FreeSpace) -> Option<FreeSpace> {
        let mut claimed;

        // search for the first item in the list that have equal or bigger space available
        let pos = match self.data.binary_search(expected_amount) {
            Ok(pos) => pos,
            Err(pos) if pos < self.data.len() => pos,
            _ => return None,
        };

        claimed = self.data.remove(pos);

        // store again the free space if the space claimed has been bigger than the space
        // that is going to be filled
        if claimed.space > expected_amount.space {
            let free_space = FreeSpace {
                space: claimed.space - expected_amount.space,
                cursor: claimed.cursor + expected_amount.space,
            };

            self.data.insert(pos, free_space);
        }

        // update the real space that is going to be retrieved (just for correctness)
        claimed.space = expected_amount.space;

        Some(claimed)
    }
}

impl Ord for FreeSpace {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut order = self.space.cmp(&other.space);
        if order == Ordering::Equal {
            // todo(): refine the ordering
        }

        return order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_free_space() {
        // test inserting one element
        let mut free_list = FreeList::new();
        free_list.insert_free_space(0, 10);
        assert_eq!(free_list.data, vec![FreeSpace {space: 10, cursor: 0}]);

        // test inserting free space at the beginning
        free_list.insert_free_space(10, 5);
        assert_eq!(
            free_list.data,
            vec![FreeSpace {space: 5, cursor: 10}, FreeSpace {space: 10, cursor: 0}]
        );

        // test inserting free space at the end
        free_list.insert_free_space(20, 80);
        assert_eq!(
            free_list.data,
            vec![
                FreeSpace {space: 5, cursor: 10},
                FreeSpace {space: 10, cursor: 0},
                FreeSpace {space: 80, cursor: 20},
            ]
        );

        // test inserting same space but different cursor
        free_list.insert_free_space(30, 8);
        assert_eq!(
            free_list.data,
            vec![
                FreeSpace {space: 5, cursor: 10},
                FreeSpace {space: 8, cursor: 30},
                FreeSpace {space: 10, cursor: 0},
                FreeSpace {space: 80, cursor: 20},
            ]
        );

        // test inserting cursor already present in the list with different space (should not happen in theory)
        free_list.insert_free_space(0, 11);
        assert_eq!(
            free_list.data,
            vec![
                FreeSpace {space: 5, cursor: 10},
                FreeSpace {space: 8, cursor: 30},
                FreeSpace {space: 10, cursor: 0},
                FreeSpace {space: 11, cursor: 0},
                FreeSpace {space: 80, cursor: 20},
            ]
        );

        // test insert same space and same cursor (can't happen in theory)
        free_list.insert_free_space(10, 5);
        assert_eq!(
            free_list.data,
            vec![
                FreeSpace {space: 5, cursor: 10},
                FreeSpace {space: 5, cursor: 10},
                FreeSpace {space: 8, cursor: 30},
                FreeSpace {space: 10, cursor: 0},
                FreeSpace {space: 11, cursor: 0},
                FreeSpace {space: 80, cursor: 20},
            ]
        );

    }

    #[test]
    fn test_retrieve_free_space() {
        let mut free_list = FreeList::new();

        // test retrieving free space when there are no values stored
        assert_eq!(free_list.retrieve_free_space(7), None);

        // test retrieving more space than available
        free_list.insert_free_space(15, 5);
        assert_eq!(free_list.retrieve_free_space(6), None);

        // test retrieving space that matches the exact same space
        free_list.insert_free_space(20, 12);
        assert_eq!(free_list.retrieve_free_space(12), Some(20));
        assert_eq!(free_list.data, vec![FreeSpace {space: 5, cursor: 15}]);

        // test that we pick the smaller space available
        free_list.insert_free_space(10, 300);
        assert_eq!(free_list.retrieve_free_space(5), Some(15));
        assert_eq!(free_list.data, vec![FreeSpace {space: 300, cursor: 10}]);

        // test that we subtract the remaining space when space asked < space available
        assert_eq!(free_list.retrieve_free_space(1), Some(10));
        assert_eq!(free_list.data, vec![FreeSpace {space: 299, cursor: 11}]);
    }

    #[test]
    fn test_retrieve_equal_or_bigger_than() {
        let mut free_list = FreeList::new();
        free_list.insert_free_space(0, 10);
        free_list.insert_free_space(15, 5);

        // test retrieving free space that is equal to the requested size
        assert_eq!(
            free_list.retrieve_equal_or_bigger_than(&FreeSpace {space: 10, cursor: 0}),
            Some(FreeSpace {space: 10, cursor: 0})
        );
        assert_eq!(free_list.data, vec![FreeSpace {space: 5, cursor: 15}]);

        // test retrieving free space that is bigger than the requested size
        assert_eq!(
            free_list.retrieve_equal_or_bigger_than(&FreeSpace {space: 12, cursor: 0}), None
        );
        assert_eq!(free_list.data, vec![FreeSpace {space: 5, cursor: 15}]);

        // test retrieving space that is smaller than available and make sure that the space
        // remaining is reinserted and updated
        assert_eq!(
            free_list.retrieve_equal_or_bigger_than(&FreeSpace {space: 1, cursor: 0}),
            Some(FreeSpace {space: 1, cursor: 15})
        );
        assert_eq!(free_list.data, vec![FreeSpace {space: 4, cursor: 16}])
    }
}
