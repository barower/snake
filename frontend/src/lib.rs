use libc::c_int;
use std::ptr;
use ncurses::*;

#[repr(C)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[repr(C)]
pub struct PointList {
    x: c_int,
    y: c_int,
    next: *mut PointList,
}

#[no_mangle]
pub unsafe extern "C" fn get_next_move(previous: Direction) -> Direction {
    let ch = getch();
    match ch {
        KEY_LEFT => { if let Direction::RIGHT = previous { previous } else { Direction::LEFT } }
        KEY_RIGHT => { if let Direction::LEFT = previous { previous } else { Direction::RIGHT } }
        KEY_DOWN => { if let Direction::UP = previous { previous } else { Direction::DOWN } }
        KEY_UP => { if let Direction::DOWN = previous { previous } else { Direction::UP } }
        _ => previous
    }
}

#[no_mangle]
pub unsafe extern "C" fn display_points(mut snake: *mut PointList, symbol: chtype) {
    while snake != ptr::null_mut() {
        mvaddch((*snake).y as i32, (*snake).x as i32, symbol);
        snake = (*snake).next;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

