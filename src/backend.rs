extern crate libc;
extern crate rand;

use libc::{c_void, malloc, free};
use std::ptr;
use std::mem::size_of;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub next: *mut Point,
}

pub struct Board {
    pub snake: *mut Point,
    pub foods: *mut Point,
    xmax: i32,
    ymax: i32,
}

impl Board {
    pub fn new(snake: *mut Point, foods: *mut Point, xmax: i32, ymax: i32) -> Board {
        Board {
            foods: foods,
            snake: snake,
            xmax: xmax,
            ymax: ymax,
        }
    }
}

fn is_same_place(cell1: &Point, cell2: &Point) -> bool {
    (cell1.x == cell2.x) && (cell1.y == cell2.y)
}

pub unsafe fn move_snake(board: &mut Board, dir: Direction) -> Option<()> {
    // Create a new beginning. Check boundaries.
    let beginning: *mut Point = next_move(board, dir)?;

    // If we've gone backwards, don't do anything
    if (*board.snake).next != ptr::null_mut() && is_same_place(&*beginning, &*(*board.snake).next) {
        (*beginning).next = ptr::null_mut();
        free(beginning as *mut c_void);
        return Some(());
    }

    // Check for collisions
    if list_contains(beginning, board.snake) {
        return None;
    }

    // Check for food
    if list_contains(beginning, board.foods) {
        // Attach the beginning to the rest of the snake;
        (*beginning).next = board.snake;
        board.snake = beginning;
        remove_from_list(beginning, &mut(board.foods));
        add_new_food(board);

        return Some(());
    }

    // Attach the beginning to the rest of the snake
    (*beginning).next = board.snake;
    board.snake = beginning;

    // Cut off the end
    let mut end: *mut Point = board.snake;
    while (*(*end).next).next != ptr::null_mut() {
        end = (*end).next;
    }
    free((*end).next as *mut c_void);
    (*end).next = ptr::null_mut();

    Some(())
}


unsafe fn next_move(board: &Board, dir: Direction) -> Option<*mut Point> {
    let snake: *mut Point = board.snake;
    let mut new_x: i32 = (*snake).x;
    let mut new_y: i32 = (*snake).y;
    match dir {
        Direction::UP => { new_y = (*snake).y - 1 },
        Direction::DOWN => { new_y = (*snake).y + 1 },
        Direction::LEFT => { new_x = (*snake).x - 1 },
        Direction::RIGHT => { new_x = (*snake).x + 1 },
    }
    if (new_x < 0) || (new_y < 0) || (new_x >= board.xmax) || (new_y >= board.ymax) {
        None
    } else {
        Some(create_cell(new_x, new_y))
    }
}


unsafe fn list_contains(cell: *mut Point, list: *mut Point) -> bool {
    let mut s: *mut Point = list;
    while s != ptr::null_mut() {
        if is_same_place(&*s, &*cell) {
            return true;
        }
        s = (*s).next;
    }
    return false;
}

unsafe fn create_cell(x: i32, y: i32) -> *mut Point {
    let cell: *mut Point = malloc(size_of::<Point>()) as *mut Point;
    (*cell).x = x;
    (*cell).y = y;
    (*cell).next = ptr::null_mut();
    cell
}

unsafe fn create_random_cell(xmax: i32, ymax: i32) -> *mut Point {
    let mut rng = rand::thread_rng();
    create_cell(rng.gen_range(0, xmax-1), rng.gen_range(0, ymax-1))
}

pub unsafe fn create_snake() -> *mut Point {
    let a: *mut Point = create_cell(2,2);
    let b: *mut Point = create_cell(2,3);
    (*a).next = b;
    a
}

/*
 * Removes from the list or returns false
 */
unsafe fn remove_from_list(elt: *mut Point, list: *mut *mut Point) -> bool {
    let mut curr_p: *mut Point = *list;
    let mut prev_p: *mut Point = ptr::null_mut();

    // Originally a for loop
    while curr_p != ptr::null_mut() {
        if is_same_place(&*curr_p, &*elt) {
            if prev_p == ptr::null_mut() {
                *list = (*curr_p).next;
            } else {
                (*prev_p).next = (*curr_p).next;
            }
            free(curr_p as *mut c_void);
            return true;
        }

        prev_p = curr_p;
        curr_p = (*curr_p).next;
    }

    false
}

pub unsafe fn add_new_food(board: *mut Board) {
    let mut new_food: *mut Point;
    loop {
        // Freed inside remove_from_list
        new_food = create_random_cell((*board).xmax, (*board).ymax);
        if !(list_contains(new_food, (*board).foods) || list_contains(new_food, (*board).snake)) { break; }
    }
    (*new_food).next = (*board).foods;
    (*board).foods = new_food;
}


#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
    */

    #[test]
    fn test_move_snake_length_1() {
        unsafe {
            let snake: *mut Point = create_cell(0, 0);
            let board: *mut Board = create_board(snake, ptr::null_mut(), 2, 2);
            assert_eq!(move_snake(&mut *board, Direction::LEFT), None);
            assert_eq!(move_snake(&mut *board, Direction::UP), None);
            assert_eq!(move_snake(&mut *board, Direction::DOWN), Some(()));
            assert_eq!(move_snake(&mut *board, Direction::DOWN), None);
        }
    }

    #[test]
    fn test_move_snake_backwards() {
        unsafe {
            let snake: *mut Point = create_cell(2, 2);
            (*snake).next = create_cell(2, 3);
            let board: *mut Board = create_board(snake, ptr::null_mut(), 2, 2);
            assert_eq!(move_snake(&mut *board, Direction::DOWN), None);
            assert_eq!((*(*board).snake).x, 2);
            assert_eq!((*(*board).snake).y, 2);
        }
    }

    #[test]
    fn test_move_snake_collision() {
        unsafe {
            let snake: *mut Point = create_cell(2, 2);
            (*snake).next = create_cell(2, 3);
            (*(*snake).next).next = create_cell(3, 3);
            (*(*(*snake).next).next).next = create_cell(3, 2);
            let board: *mut Board = create_board(snake, ptr::null_mut(), 4, 4);
            assert_eq!(move_snake(&mut *board, Direction::RIGHT), None);
        }
    }

    #[test]
    fn test_move_snake_down() {
        unsafe {
            let snake: *mut Point = create_cell(2, 2);
            let board: *mut Board = create_board(snake, ptr::null_mut(), 4, 4);
            assert_eq!(move_snake(&mut *board, Direction::DOWN), Some(()));
            assert_eq!((*(*board).snake).x, 2);
            assert_eq!((*(*board).snake).y, 3);
        }
    }

    #[test]
    fn test_move_snake_normally() {
        unsafe {
            let snake: *mut Point = create_cell(2, 2);
            (*snake).next = create_cell(2, 3);
            let board: *mut Board = create_board(snake, ptr::null_mut(), 4, 4);
            assert_eq!(move_snake(&mut *board, Direction::UP), Some(()));
            assert_eq!((*(*board).snake).x, 2);
            assert_eq!((*(*board).snake).y, 1);
            assert_eq!((*(*(*board).snake).next).x, 2);
            assert_eq!((*(*(*board).snake).next).y, 2);
            assert_eq!((*(*(*board).snake).next).next, ptr::null_mut());
        }
    }
}

