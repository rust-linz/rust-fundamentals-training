use std::ffi::c_double;

/// An ellipse with semi-axes `a` and `b`.
#[repr(C)]
pub struct Ellipse {
    pub a: c_double,
    pub b: c_double,
}

/// Calculates the area of an ellipse.
///
/// Returns NaN if `ellipse` is NULL. Otherwise, `ellipse` must point to a
/// valid, properly aligned `Ellipse`.
//
// `Option<&Ellipse>` has the same ABI as a nullable C pointer: NULL becomes
// `None`. A plain `&Ellipse` would promise Rust that the pointer is never
// NULL, but nothing on the C side enforces that, so a NULL from C would be
// undefined behavior.
//
// If a function exported with `extern "C"` panics, the process aborts
// (unwinding into C is not allowed). This function cannot panic.
#[unsafe(no_mangle)]
pub extern "C" fn area_ellipse(ellipse: Option<&Ellipse>) -> c_double {
    match ellipse {
        Some(e) => e.a * e.b * std::f64::consts::PI,
        None => c_double::NAN,
    }
}
