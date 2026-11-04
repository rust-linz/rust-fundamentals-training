use std::ffi::c_double;

#[repr(C)]
pub struct Ellipse {
    pub a: c_double,
    pub b: c_double,
}

#[unsafe(no_mangle)]
pub extern "C" fn area_ellipse(ellipse: &Ellipse) -> c_double {
    ellipse.a * ellipse.b * std::f64::consts::PI
}
