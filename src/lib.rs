extern crate libc;
use libc::{c_int, malloc};
use std::ptr;
use std::mem::size_of;

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

#[no_mangle]
pub unsafe extern fn list_contains(cell: *mut PointList, list: *mut PointList) -> bool {
    let mut s: *mut PointList = list;
    while s != ptr::null_mut() {
        if is_same_place(s, cell) {
            return true;
        }
        s = (*s).next;
    }
    return false;
}

#[no_mangle]
pub unsafe extern fn create_cell(x: c_int, y: c_int) -> *mut PointList {
    let cell: *mut PointList = malloc(size_of::<PointList>()) as *mut PointList;
    (*cell).x = x;
    (*cell).y = y;
    (*cell).next = ptr::null_mut();
    cell
}
