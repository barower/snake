extern crate libc;
use libc::c_int;

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
    x: c_int,
    y: c_int,
    next: *mut PointList,
}

#[repr(C)]
pub struct Board {
    snake: *mut PointList,
    foods: *mut PointList,
    xmax: c_int,
    ymax: c_int,
}

#[no_mangle]
pub unsafe extern fn is_same_place(cell1: *mut PointList, cell2: *mut PointList) -> bool {
    ((*cell1).x == (*cell2).x) && ((*cell1).y == (*cell2).y)
}

