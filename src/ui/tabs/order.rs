#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Next,
    Previous,
}

pub fn neighbor(current: u32, count: u32, direction: Direction) -> Option<u32> {
    if count == 0 {
        return None;
    }
    let current = current % count;
    let target = match direction {
        Direction::Next => (current + 1) % count,
        Direction::Previous => (current + count - 1) % count,
    };
    Some(target)
}

#[cfg(test)]
mod tests {
    use super::{Direction, neighbor};

    #[test]
    fn moves_to_adjacent_tab() {
        assert_eq!(neighbor(1, 3, Direction::Next), Some(2));
        assert_eq!(neighbor(1, 3, Direction::Previous), Some(0));
    }

    #[test]
    fn wraps_around_the_ends() {
        assert_eq!(neighbor(2, 3, Direction::Next), Some(0));
        assert_eq!(neighbor(0, 3, Direction::Previous), Some(2));
    }

    #[test]
    fn stays_on_single_tab() {
        assert_eq!(neighbor(0, 1, Direction::Next), Some(0));
        assert_eq!(neighbor(0, 1, Direction::Previous), Some(0));
    }

    #[test]
    fn has_no_neighbor_without_tabs() {
        assert_eq!(neighbor(0, 0, Direction::Next), None);
    }
}
