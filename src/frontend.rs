use ncurses::*;

use crate::backend::*;

pub fn get_next_move(previous: Direction) -> Direction {
    let ch = getch();
    match ch {
        KEY_LEFT => { if let Direction::RIGHT = previous { previous } else { Direction::LEFT } }
        KEY_RIGHT => { if let Direction::LEFT = previous { previous } else { Direction::RIGHT } }
        KEY_DOWN => { if let Direction::UP = previous { previous } else { Direction::DOWN } }
        KEY_UP => { if let Direction::DOWN = previous { previous } else { Direction::UP } }
        _ => previous
    }
}

pub fn display_points(list: &PointList, symbol: chtype) {
    for point in list.list.iter() {
        mvaddch(point.y, point.x, symbol);
    }
}

