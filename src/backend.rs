extern crate libc;
extern crate rand;

use libc::{c_int, c_void, malloc, free};
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

pub enum Status {
    SUCCESS,
    FAILURE,
}

pub struct PointList {
    pub x: c_int,
    pub y: c_int,
    pub next: *mut PointList,
}

pub struct Board {
    pub snake: *mut PointList,
    pub foods: *mut PointList,
    xmax: c_int,
    ymax: c_int,
}

fn is_same_place(cell1: &PointList, cell2: &PointList) -> bool {
    (cell1.x == cell2.x) && (cell1.y == cell2.y)
}

pub unsafe fn move_snake(board: *mut Board, dir: Direction) -> Status {
    // Create a new beginning. Check boundaries.
    let beginning: *mut PointList = next_move(board, dir);
    if beginning == ptr::null_mut() {
        return Status::FAILURE;
    }

    // If we've gone backwards, don't do anything
    if (*(*board).snake).next != ptr::null_mut() && is_same_place(&*beginning, &*(*(*board).snake).next) {
        (*beginning).next = ptr::null_mut();
        free(beginning as *mut c_void);
        return Status::SUCCESS;
    }

    // Check for collisions
    if list_contains(beginning, (*board).snake) {
        return Status::FAILURE;
    }

    // Check for food
    if list_contains(beginning, (*board).foods) {
        // Attach the beginning to the rest of the snake;
        (*beginning).next = (*board).snake;
        (*board).snake = beginning;
        remove_from_list(beginning, &mut((*board).foods));
        add_new_food(board);

        return Status::SUCCESS;
    }

    // Attach the beginning to the rest of the snake
    (*beginning).next = (*board).snake;
    (*board).snake = beginning;

    // Cut off the end
    let mut end: *mut PointList = (*board).snake;
    while (*(*end).next).next != ptr::null_mut() {
        end = (*end).next;
    }
    free((*end).next as *mut c_void);
    (*end).next = ptr::null_mut();

    return Status::SUCCESS;
}


pub unsafe fn next_move(board: *mut Board, dir: Direction) -> *mut PointList {
    let snake: *mut PointList = (*board).snake;
    let mut new_x: c_int = (*snake).x;
    let mut new_y: c_int = (*snake).y;
    match dir {
        Direction::UP => { new_y = (*snake).y - 1 },
        Direction::DOWN => { new_y = (*snake).y + 1 },
        Direction::LEFT => { new_x = (*snake).x - 1 },
        Direction::RIGHT => { new_x = (*snake).x + 1 },
    }
    if (new_x < 0) || (new_y < 0) || (new_x >= (*board).xmax) || (new_y >= (*board).ymax) {
        ptr::null_mut()
    } else {
        create_cell(new_x, new_y)
    }
}


pub unsafe fn list_contains(cell: *mut PointList, list: *mut PointList) -> bool {
    let mut s: *mut PointList = list;
    while s != ptr::null_mut() {
        if is_same_place(&*s, &*cell) {
            return true;
        }
        s = (*s).next;
    }
    return false;
}

pub unsafe fn create_cell(x: c_int, y: c_int) -> *mut PointList {
    let cell: *mut PointList = malloc(size_of::<PointList>()) as *mut PointList;
    (*cell).x = x;
    (*cell).y = y;
    (*cell).next = ptr::null_mut();
    cell
}

pub unsafe fn create_random_cell(xmax: c_int, ymax: c_int) -> *mut PointList {
    let mut rng = rand::thread_rng();
    create_cell(rng.gen_range(0, xmax-1), rng.gen_range(0, ymax-1))
}

pub unsafe fn create_snake() -> *mut PointList {
    let a: *mut PointList = create_cell(2,2);
    let b: *mut PointList = create_cell(2,3);
    (*a).next = b;
    a
}

pub unsafe fn create_board(snake: *mut PointList, foods: *mut PointList, xmax: c_int, ymax: c_int) -> *mut Board {
    let board: *mut Board = malloc(size_of::<Board>()) as *mut Board;
    (*board).foods = foods;
    (*board).snake = snake;
    (*board).xmax = xmax;
    (*board).ymax = ymax;
    board
}

/*
 * Removes from the list or returns false
 */
pub unsafe fn remove_from_list(elt: *mut PointList, list: *mut *mut PointList) -> bool {
    let mut curr_p: *mut PointList = *list;
    let mut prev_p: *mut PointList = ptr::null_mut();

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
    let mut new_food: *mut PointList;
    loop {
        // Ouch
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
}

