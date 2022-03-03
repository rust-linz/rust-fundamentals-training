// region: External dependencies
// Required because of bug in wasm_bindgen 0.2.79
// See also https://github.com/rustwasm/wasm-bindgen/issues/2774
#![allow(clippy::unused_unit)]

use std::fmt::Display;
use rand::Rng;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String); // Import console.log function
}
// endregion

// region: Initialization function
#[wasm_bindgen(start)]
pub fn run() {
    // Enable forwarding panic messages to console.error. Read more about this function at
    // https://github.com/rustwasm/console_error_panic_hook. 
    // If you want to really optimize your Wasm size to the very last bit, avoid
    // panicing (read more e.g. at https://rustwasm.github.io/book/reference/code-size.html#avoid-panicking).
    console_error_panic_hook::set_once();

    // Use console.log to display a status message. Note: UTF8 works 👍
    log("🦀 Rust Wasm initialized 🤘".to_string());
}
// endregion

// region: Operators
#[derive(Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operator::Plus => '+',
            Operator::Minus => '-',
            Operator::Multiply => '*',
            Operator::Divide => '/',
        })
    }
}

impl Operator {
    fn is_muldiv(&self) -> bool {
        matches!(self, Operator::Multiply | Operator::Divide)
    }

    fn is_plusminus(&self) -> bool {
        !self.is_muldiv()
    }

    fn calculate(&self, left: i32, right: i32) -> Result<i32, ()> {
        match self {
            Operator::Multiply => Ok(left * right),
            Operator::Divide => {
                if left % right != 0 {
                    // Rule: Divisions with remainder are not allowed
                    Err(())
                } else {
                    Ok(left / right)
                }
            }
            Operator::Plus => Ok(left + right),
            Operator::Minus => Ok(left - right),
        }
    }
}
// endregion

// region: Challenge data structure
#[derive(Serialize, Deserialize, Default)]
pub struct Challenge {
    pub formula: String,
    pub result: i32,
}
// endregion

// region: Generate challenge
#[wasm_bindgen(js_name = generateChallenge)]
pub fn generate_challenge() -> Result<JsValue, JsValue> {
    let mut rng = rand::thread_rng();
    let mut challenge: Challenge = Default::default();
    loop {
        let mut values = [0i32; 3];
        for v in &mut values {
            *v = rng.gen_range(if rand::random::<bool>() {
                // Single-digit value
                1..10
            } else {
                // Double-digit value
                10..100
            });
        }

        let mut ops = [Operator::Plus; 2];
        for o in &mut ops {
            *o = match rng.gen_range(0u8..4) {
                0 => Operator::Plus,
                1 => Operator::Minus,
                2 => Operator::Multiply,
                3 => Operator::Divide,
                _ => panic!("This should never happen"),
            };
        }

        challenge.formula = format!("{}{}{}{}{}", values[0], ops[0], values[1], ops[1], values[2]);
        if challenge.formula.len() != 7 {
            // Rule: Formula must always have a length of 7
            continue;
        }

        fn calculate(ops: &[Operator; 2], values: &[i32; 3]) -> Result<i32, ()> {
            if ops[0].is_muldiv() || (ops[0].is_plusminus() && ops[1].is_plusminus()) {
                ops[1].calculate(ops[0].calculate(values[0], values[1])?, values[2])
            } else {
                ops[0].calculate(values[0], ops[1].calculate(values[1], values[2])?)
            }
        }

        match calculate(&ops, &values) {
            Ok(r) => challenge.result = r,
            Err(()) => continue
        };

        if !(0..100).contains(&challenge.result) {
            // Rule: Result must not be negative and must be < 100
            continue;
        }

        break;
    }
    
    Ok(serde_wasm_bindgen::to_value(&challenge)?)
}
// endregion

#[cfg(test)]
mod tests {
    extern crate wasm_bindgen_test;
    
    use super::*;
    use wasm_bindgen_test::*;
    use evalexpr::*;

    // region: Operator tests
    #[wasm_bindgen_test]
    fn display() {
        assert_eq!("+", format!("{}", super::Operator::Plus));
    }

    #[wasm_bindgen_test]
    fn operator_calculate_plus() {
        assert_eq!(super::Operator::Plus.calculate(1, 2), Ok(3));
    }

    #[wasm_bindgen_test]
    fn operator_calculate_div() {
        assert_eq!(super::Operator::Divide.calculate(4, 2), Ok(2));
        assert!(super::Operator::Divide.calculate(5, 2).is_err());
    }
    // endregion

    // region: Generation tests
    #[wasm_bindgen_test]
    fn generate() {
        // Generate 100 formula and make sure result is correct
        for _ in 0..100 {
            if let Ok(res) = generate_challenge() {
                if let Ok(c) = serde_wasm_bindgen::from_value::<Challenge>(res) {
                    assert_eq!(c.formula.len(), 7, "Invalid length: {}", c.formula);
                    assert!(c.result >= 0 || c.result < 100, "Invalid result range: {}", c.formula);
                    assert_eq!(Ok(Value::Int(c.result as i64)), eval(&c.formula), "Invalid result: {}", c.formula);
                    continue;
                }
            }

            assert!(false);
        }
    }
    // endregion
}
