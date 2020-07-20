use ncurses::*;
use std::ptr;

mod frontend;
mod backend;

fn main() {
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(100);

    let mut xmax: i32 = 0;
    let mut ymax: i32 = 0;
    getmaxyx(stdscr(), &mut ymax, &mut xmax);
    let mut dir: backend::Direction = backend::Direction::RIGHT;

    unsafe {
        let board: *mut backend::Board = backend::create_board(backend::create_snake(), ptr::null_mut(), xmax, ymax);
        for _i in 1..=6 {
            backend::add_new_food(board);
        }

        loop {
            clear();
            frontend::display_points((*board).snake, ACS_BLOCK());
            frontend::display_points((*board).foods, ACS_DIAMOND());
            refresh();
            dir = frontend::get_next_move(dir);
            if let Err(()) = backend::move_snake(&mut *board, dir) { break; }
        }
    }

    endwin();
}
