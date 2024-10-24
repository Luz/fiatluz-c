// Many functions might receive null pointers from the user.
// Maybe just use pure Rust in the future?
#![allow(clippy::missing_safety_doc)]

use fiatluz::*;

#[no_mangle]
pub extern "C" fn point(x: i64, y: i64) -> Point {
    println!("Rust: point({}, {})", x, y);
    Point { x, y }
}

#[no_mangle]
pub extern "C" fn newPath() -> *mut Path {
    println!("Rust: newPath()");
    let h = Box::new(Path::default());
    Box::into_raw(h)
}
/*
#[no_mangle]
pub extern "C" fn newPathFromZero() -> *mut Path {
    println!("Rust: newPathFromZero()");
    let p = Point { x: 0, y: 0 };
    let path = Path { path: vec![p] };
    Box::into_raw(Box::new(path))
}
*/

#[no_mangle]
pub unsafe extern "C" fn deletePath(h: *mut Path) {
    if h.is_null() {
        println!("Received a null pointer :(");
    }
    let owned = unsafe { Box::from_raw(h) };
    println!("Rust: deletePath: {:?}", owned);
    // "drop(owned);" is not necessary:
    // Rust frees the data on the heap as soon as the pointer goes out of scope
}

#[no_mangle]
pub unsafe extern "C" fn addPointToPath(p: Point, handle: *mut Path) {
    if handle.is_null() {
        println!("Received a null pointer :(");
    }
    let path: &mut Path;

    // Dereference the raw pointer to get our desired object
    unsafe {
        path = &mut *handle;
    }

    println!("Rust: addPointToPath({:?}, {:?})", p, path);
    println!("{:?}", path);
    add_point_to_path(p, path);
    println!("{:?}", path);
}
