#[repr(C)]
pub struct Ellipse {
    pub a: cty::c_double,
    pub b: cty::c_double,
}

#[no_mangle]
pub extern "C" fn area_ellipse(ellipse: &Ellipse) -> cty::c_double {
    ellipse.a * ellipse.b * std::f64::consts::PI
}
