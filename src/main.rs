use ncurses::*;

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

    let mut board = Box::new(backend::Board::new(backend::PointList::new_snake(), backend::PointList::new_empty(), xmax, ymax));
    for _i in 1..=6 {
        board.add_new_food();
    }

    loop {
        clear();
        frontend::display_points(&board.snake, ACS_BLOCK());
        frontend::display_points(&board.foods, ACS_DIAMOND());
        refresh();
        dir = frontend::get_next_move(dir);
        if let None = board.move_snake(dir) { break; }
    }

    endwin();
}
