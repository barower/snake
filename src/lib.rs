extern crate libc;

#[repr(C)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[repr(C)]
pub enum Status {
    SUCCESS,
    FAILURE,
}

#[repr(C)]
pub struct PointList {
    x: isize,
    y: isize,
    next: *mut PointList,
}

#[repr(C)]
pub struct Board {
    snake: *mut PointList,
    foods: *mut PointList,
    xmax: isize,
    ymax: isize,
}

