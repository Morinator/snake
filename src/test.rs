
#[cfg(test)]
mod tests {
    mod snake {
        use crate::snake;

        #[test]
        fn array_direction_up() {
            assert_eq!(snake::Direction::Up.as_array(), [0, -1] )
        }

        #[test]
        fn array_direction_down() {
            assert_eq!(snake::Direction::Down.as_array(), [0, 1] )
        }

        #[test]
        fn array_direction_left() {
            assert_eq!(snake::Direction::Left.as_array(), [-1, 0] )
        }

        #[test]
        fn array_direction_right() {
            assert_eq!(snake::Direction::Right.as_array(), [1, 0] )
        }

        #[test]
        fn opposite_direction_up() {
            assert_eq!(snake::Direction::Up.opposite(), snake::Direction::Down )
        }

        #[test]
        fn opposite_direction_down() {
            assert_eq!(snake::Direction::Down.opposite(), snake::Direction::Up )
        }

        #[test]
        fn opposite_direction_left() {
            assert_eq!(snake::Direction::Left.opposite(), snake::Direction::Right )
        }

        #[test]
        fn opposite_direction_right() {
            assert_eq!(snake::Direction::Right.opposite(), snake::Direction::Left )
        }
    }
}