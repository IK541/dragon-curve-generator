#[derive(Clone,Copy)]
pub enum Direction{
    Up,
    Right,
    Down,
    Left,
}

impl Direction{
    pub fn clockwise(self)->Direction{
        match self{
            Direction::Up => Direction::Right,
            Direction:: Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    pub fn counterclockwise(self)->Direction{
        match self{
            Direction::Up => Direction::Left,
            Direction:: Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

// pub fn move_i32 (direction: &mut Direction, x: &mut i32, y: &mut i32) {
//     match direction {
//         Direction::Up => *y += 1,
//         Direction::Right => *x += 1,
//         Direction::Down => *y -= 1,
//         Direction::Left => *x -= 1,
//     };
// }
pub fn move_u32 (direction: &mut Direction, x: &mut u32, y: &mut u32) {
    match direction {
        Direction::Up => *y += 1,
        Direction::Right => *x += 1,
        Direction::Down => *y -= 1,
        Direction::Left => *x -= 1,
    };
}
