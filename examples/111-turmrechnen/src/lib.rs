use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Tower {
    values: Vec<i32>,
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
impl Tower {
    pub fn new(base: i32, height: i32) -> Tower {
        let mut base = base;
        let mut result = Tower { values: vec![0; (height as usize - 1) * 2 * 3], };

        let mut ix = 0;
        for i in 2..=height {
            result.values[ix] = base;
            result.values[ix + 1] = i;
            base *= i;
            result.values[ix + 2] = base;
            ix += 3;
        }

        for i in 2..=height {
            result.values[ix] = base;
            result.values[ix + 1] = i;
            base /= i;
            result.values[ix + 2] = base;
            ix += 3;
        }

        println!("{:?}", result.values);

        result
    }

    pub fn tower(&self) -> *const i32 {
        self.values.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_tower() {
        let tower = super::Tower::new(1, 3);
        assert_eq!(tower.values, vec![1, 2, 2, 2, 3, 6, 6, 2, 3, 3, 3, 1]);
    }
}