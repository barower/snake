extern crate libc;
extern crate rand;

use libc::{c_void, malloc, free};
use std::{ptr, collections::VecDeque};
use std::mem::size_of;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    fn new_random(xmax: i32, ymax: i32) -> Point {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0, xmax-1),
            y: rng.gen_range(0, ymax-1),
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

pub struct PointList {
    pub list: VecDeque<Point>,
}

impl PointList {

    pub fn new_empty() -> PointList {
        let list: VecDeque<Point> = VecDeque::new();
        PointList {
            list: list,
        }
    }

    pub fn new_snake() -> PointList {
        let mut list: VecDeque<Point> = VecDeque::new();
        list.push_front(Point::new(2,3));
        list.push_front(Point::new(2,2));
        PointList {
            list: list,
        }
    }

    fn add_beginning(&mut self, p: &Point) {
        self.list.push_front(p.clone());
    }

    fn remove_back(&mut self) {
        self.list.pop_back();
    }

    fn contains(&mut self, p: &Point) -> bool {
        self.list.contains(&p)
    }

    fn remove(&mut self, p: &Point) {
        self.list.retain(|x| *x != *p);
    }

}

pub struct Board {
    pub snake: PointList,
    pub foods: PointList,
    xmax: i32,
    ymax: i32,
}

impl Board {
    pub fn new(snake: PointList, foods: PointList, xmax: i32, ymax: i32) -> Board {
        Board {
            foods: foods,
            snake: snake,
            xmax: xmax,
            ymax: ymax,
        }
    }

    unsafe fn next_move(&mut self, dir: Direction) -> Option<Point> {
        let snake_head: &Point = &self.snake.list[0];
        let mut new_x: i32 = snake_head.x;
        let mut new_y: i32 = snake_head.y;
        match dir {
            Direction::UP => { new_y = snake_head.y - 1 },
            Direction::DOWN => { new_y = snake_head.y + 1 },
            Direction::LEFT => { new_x = snake_head.x - 1 },
            Direction::RIGHT => { new_x = snake_head.x + 1 },
        }
        if (new_x < 0) || (new_y < 0) || (new_x >= self.xmax) || (new_y >= self.ymax) {
            None
        } else {
            Some(Point::new(new_x, new_y))
        }
    }

    pub unsafe fn move_snake(&mut self, dir: Direction) -> Option<()> {
        // Create a new beginning. Check boundaries.
        let beginning: Point = self.next_move(dir)?;

        // If we've gone backwards, don't do anything
        if self.snake.list.len() > 1 && beginning == self.snake.list[1] {
            return Some(());
        }

        // Check for collisions
        if self.snake.contains(&beginning) {
            return None;
        }

        // Check for food
        if self.foods.contains(&beginning) {
            // Attach the beginning to the rest of the snake;
            self.snake.add_beginning(&beginning);
            self.foods.remove(&beginning);
            self.add_new_food();

            return Some(());
        }

        // Attach the beginning to the rest of the snake
        self.snake.add_beginning(&beginning);

        // Cut off the end
        self.snake.remove_back();

        Some(())
    }

    pub unsafe fn add_new_food(&mut self) {
        let mut new_food: Point;
        loop {
            // Freed inside remove_from_list
            new_food = Point::new_random(self.xmax, self.ymax);
            if !(self.foods.contains(&new_food) || self.snake.contains(&new_food)) { break; }
        }
        self.foods.add_beginning(&new_food);
    }

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
    fn test_board_new() {
        unsafe {
            let snake: *mut Point = create_cell(0, 0);
            let board = Box::new(Board::new(snake, ptr::null_mut(), 2, 3));
            assert_eq!(board.xmax, 2);
            assert_eq!(board.ymax, 3);
            assert_ne!(board.snake, ptr::null_mut());
            assert_eq!((*board.snake).x, 0);
        }
    }

    #[test]
    fn test_move_snake_length_1() {
        unsafe {
            let snake: *mut Point = create_cell(0, 0);
            let mut board = Box::new(Board::new(snake, ptr::null_mut(), 2, 2));
            assert_eq!(board.move_snake(Direction::LEFT), None);
            assert_eq!(board.move_snake(Direction::UP), None);
            assert_eq!(board.move_snake(Direction::DOWN), Some(()));
            assert_eq!(board.move_snake(Direction::DOWN), None);
        }
    }

    #[test]
    fn test_move_snake_backwards() {
        unsafe {
            let snake: *mut Point = create_cell(2, 2);
            (*snake).next = create_cell(2, 3);
            let mut board = Box::new(Board::new(snake, ptr::null_mut(), 2, 2));
            assert_eq!(board.move_snake(Direction::DOWN), None);
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
            let mut board = Box::new(Board::new(snake, ptr::null_mut(), 4, 4));
            assert_eq!(board.move_snake(Direction::RIGHT), None);
        }
    }

    #[test]
    fn test_move_snake_down() {
        unsafe {
            let snake: *mut Point = create_cell(2, 2);
            let mut board = Box::new(Board::new(snake, ptr::null_mut(), 4, 4));
            assert_eq!(board.move_snake(Direction::DOWN), Some(()));
            assert_eq!((*(*board).snake).x, 2);
            assert_eq!((*(*board).snake).y, 3);
        }
    }

    #[test]
    fn test_move_snake_normally() {
        unsafe {
            let snake: *mut Point = create_cell(2, 2);
            (*snake).next = create_cell(2, 3);
            let mut board = Box::new(Board::new(snake, ptr::null_mut(), 4, 4));
            assert_eq!(board.move_snake(Direction::UP), Some(()));
            assert_eq!((*(*board).snake).x, 2);
            assert_eq!((*(*board).snake).y, 1);
            assert_eq!((*(*(*board).snake).next).x, 2);
            assert_eq!((*(*(*board).snake).next).y, 2);
            assert_eq!((*(*(*board).snake).next).next, ptr::null_mut());
        }
    }

    #[test]
    fn test_move_snake_with_food() {
        unsafe {
            let snake: *mut Point = create_cell(1, 2);
            let foods: *mut Point = create_cell(2, 2);
            let mut board = Box::new(Board::new(snake, foods, 10, 10));
            board.move_snake(Direction::RIGHT);
            assert_eq!((*board.snake).x, 2);
            assert_eq!((*board.snake).y, 2);
            assert_ne!((*board.snake).next, ptr::null_mut());
            assert_eq!((*(*board.snake).next).x, 1);
            assert_eq!((*(*board.snake).next).y, 2);

            assert!((*board.foods).x != 2 || (*board.foods).y != 2);
        }
    }


    #[test]
    fn test_add_new_food_null() {
        unsafe {
            let snake: *mut Point = create_cell(4, 2);
            (*snake).next = create_cell(4,3);
            let mut board = Box::new(Board::new(snake, ptr::null_mut(), 20, 10));
            board.add_new_food();
            let foods: *mut Point = board.foods;
            assert_ne!(foods, ptr::null_mut());
            assert_eq!((*foods).next, ptr::null_mut());
            assert!(!list_contains(foods, snake));
        }
    }

    #[test]
    fn test_add_new_food() {
        unsafe {
            let snake: *mut Point = create_cell(4, 2);
            (*snake).next = create_cell(4,3);
            let foods: *mut Point = create_cell(3, 3);
            let mut board = Box::new(Board::new(snake, foods, 20, 10));
            board.add_new_food();
            assert_ne!((*board.foods).next, ptr::null_mut());
            assert_eq!((*(*board.foods).next).next, ptr::null_mut());
        }
    }

    #[test]
    fn test_next_move_corner() {
        unsafe {
            let snake: *mut Point = create_cell(0, 0);
            let mut board = Box::new(Board::new(snake, ptr::null_mut(), 2, 2));
            assert_eq!(board.next_move(Direction::UP), None);
            assert_eq!(board.next_move(Direction::LEFT), None);
            assert_ne!(board.next_move(Direction::DOWN), None);
            assert_ne!(board.next_move(Direction::RIGHT), None);
        }
    }

    #[test]
    fn test_next_move_bottom() {
        unsafe {
            let snake: *mut Point = create_cell(0, 1);
            let mut board = Box::new(Board::new(snake, ptr::null_mut(), 2, 2));
            assert_eq!(board.next_move(Direction::LEFT), None);
            assert_eq!(board.next_move(Direction::DOWN), None);
            assert_ne!(board.next_move(Direction::UP), None);
            assert_ne!(board.next_move(Direction::RIGHT), None);
        }
    }

    #[test]
    fn test_next_move_left() {
        unsafe {
            let snake: *mut Point = create_cell(1, 2);
            let mut board = Box::new(Board::new(snake, ptr::null_mut(), 4, 4));
            let moved: *mut Point = board.next_move(Direction::LEFT).unwrap();
            assert_eq!((*moved).x, 0);
            assert_eq!((*moved).y, 2);
        }
    }

    #[test]
    fn test_next_move_up() {
        unsafe {
            let snake: *mut Point = create_cell(2, 2);
            let mut board = Box::new(Board::new(snake, ptr::null_mut(), 4, 4));
            let moved: *mut Point = board.next_move(Direction::UP).unwrap();
            assert_eq!((*moved).x, 2);
            assert_eq!((*moved).y, 1);
        }
    }

}

