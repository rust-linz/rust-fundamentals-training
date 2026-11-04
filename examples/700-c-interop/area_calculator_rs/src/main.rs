use std::ffi::c_double;

#[repr(C)]
pub struct Ellipse {
    pub a: c_double,
    pub b: c_double,
}

unsafe extern "C" {
    pub fn area_ellipse(ellipse: *mut Ellipse, area: *mut f64);
}

fn main() {
    let mut ellipse = Ellipse { a: 3.0, b: 4.0 };
    let mut area = 0.0;
    unsafe { area_ellipse(&mut ellipse, &mut area) };
    println!("area = {}", area);
}
