use ncurses::*;
use snake_backend::*;
use std::ptr;

mod frontend;

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
    let dir: Direction = Direction::RIGHT;

    unsafe {
        let board: *mut Board = create_board(create_snake(), ptr::null_mut(), xmax, ymax);
        for _i in 1..=6 {
            add_new_food(board);
        }

        loop {
            clear();
            frontend::display_points((*board).snake, ACS_BLOCK());
            frontend::display_points((*board).foods, ACS_DIAMOND());
            refresh();
            let dir: Direction = frontend::get_next_move(dir);
            let status: Status = move_snake(board, dir.clone());
            if let Status::FAILURE = status { break; }
        }
    }

    endwin();
}
