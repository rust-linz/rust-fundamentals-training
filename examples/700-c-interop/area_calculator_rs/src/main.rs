use std::ffi::c_double;

/// Mirrors `ellipse_t` from `area_calculator.h`. `#[repr(C)]` guarantees
/// the same field order and layout as the C struct.
#[repr(C)]
pub struct Ellipse {
    pub a: c_double,
    pub b: c_double,
}

/// Raw bindings to the C library. Calling these is `unsafe` because the
/// compiler cannot check what the C code does with the pointers.
mod ffi {
    use super::Ellipse;
    use std::ffi::c_double;

    unsafe extern "C" {
        pub fn area_ellipse(ellipse: *const Ellipse, area: *mut c_double);
    }
}

/// Safe wrapper around the C function. All `unsafe` code stays in here,
/// callers can use a normal Rust API.
pub fn area_ellipse(ellipse: &Ellipse) -> f64 {
    let mut area = 0.0;
    // SAFETY: Both pointers come from Rust references, so they are non-null,
    // aligned and valid for the duration of the call. The C function does
    // not keep the pointers after it returns.
    unsafe { ffi::area_ellipse(ellipse, &mut area) };
    area
}

fn main() {
    let ellipse = Ellipse { a: 3.0, b: 4.0 };
    println!("area = {}", area_ellipse(&ellipse));
}
