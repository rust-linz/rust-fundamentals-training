// region: External dependencies
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str); // Import console.log function
}
// endregion

// region: Initialization function
// Learning: wasm-bindgen calls the function marked with `start` automatically
//           when the Wasm module is loaded.
#[wasm_bindgen(start)]
pub fn run() {
    // Enable forwarding panic messages to console.error. Read more about this function at
    // https://github.com/rustwasm/console_error_panic_hook.
    // If you want to really optimize your Wasm size to the very last bit, avoid
    // panicking (read more e.g. at https://rustwasm.github.io/book/reference/code-size.html#avoid-panicking).
    console_error_panic_hook::set_once();

    // Use console.log to display a status message. Note: UTF8 works 👍
    log("🦀 Rust Wasm initialized 🤘");
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

const OPERATORS: [Operator; 4] = [
    Operator::Plus,
    Operator::Minus,
    Operator::Multiply,
    Operator::Divide,
];

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = match self {
            Operator::Plus => '+',
            Operator::Minus => '-',
            Operator::Multiply => '*',
            Operator::Divide => '/',
        };
        write!(f, "{symbol}")
    }
}

impl Operator {
    fn is_muldiv(&self) -> bool {
        matches!(self, Operator::Multiply | Operator::Divide)
    }

    fn is_plusminus(&self) -> bool {
        !self.is_muldiv()
    }

    /// Applies the operator, returns `None` if the calculation breaks a rule of the game.
    fn calculate(&self, left: i32, right: i32) -> Option<i32> {
        match self {
            Operator::Multiply => Some(left * right),
            Operator::Divide => {
                if left % right != 0 {
                    // Rule: Divisions with remainder are not allowed
                    None
                } else {
                    Some(left / right)
                }
            }
            Operator::Plus => Some(left + right),
            Operator::Minus => Some(left - right),
        }
    }
}
// endregion

// region: Challenge data structure
#[derive(Serialize, Deserialize)]
pub struct Challenge {
    pub formula: String,
    pub result: i32,
}
// endregion

// region: Generate challenge
/// Evaluates `values[0] ops[0] values[1] ops[1] values[2]`, respecting operator precedence.
fn evaluate(ops: &[Operator; 2], values: &[i32; 3]) -> Option<i32> {
    if ops[0].is_muldiv() || ops[1].is_plusminus() {
        ops[1].calculate(ops[0].calculate(values[0], values[1])?, values[2])
    } else {
        ops[0].calculate(values[0], ops[1].calculate(values[1], values[2])?)
    }
}

#[wasm_bindgen(js_name = generateChallenge)]
pub fn generate_challenge() -> Result<JsValue, JsValue> {
    let challenge = loop {
        let values: [i32; 3] = std::array::from_fn(|_| {
            if rand::random::<bool>() {
                // Single-digit value
                rand::random_range(1..10)
            } else {
                // Double-digit value
                rand::random_range(10..100)
            }
        });

        let ops: [Operator; 2] =
            std::array::from_fn(|_| OPERATORS[rand::random_range(0..OPERATORS.len())]);

        let formula = format!(
            "{}{}{}{}{}",
            values[0], ops[0], values[1], ops[1], values[2]
        );
        if formula.len() != 7 {
            // Rule: Formula must always have a length of 7
            continue;
        }

        let Some(result) = evaluate(&ops, &values) else {
            continue;
        };

        if !(0..100).contains(&result) {
            // Rule: Result must not be negative and must be < 100
            continue;
        }

        break Challenge { formula, result };
    };

    Ok(serde_wasm_bindgen::to_value(&challenge)?)
}
// endregion

#[cfg(test)]
mod tests {
    use super::*;
    use evalexpr::{Value, eval};
    use wasm_bindgen_test::*;

    // region: Operator tests
    #[wasm_bindgen_test]
    fn display() {
        assert_eq!("+", format!("{}", Operator::Plus));
    }

    #[wasm_bindgen_test]
    fn operator_calculate_plus() {
        assert_eq!(Operator::Plus.calculate(1, 2), Some(3));
    }

    #[wasm_bindgen_test]
    fn operator_calculate_div() {
        assert_eq!(Operator::Divide.calculate(4, 2), Some(2));
        assert_eq!(Operator::Divide.calculate(5, 2), None);
    }
    // endregion

    // region: Generation tests
    #[wasm_bindgen_test]
    fn evaluate_respects_precedence() {
        use Operator::*;
        assert_eq!(evaluate(&[Plus, Multiply], &[2, 3, 4]), Some(14));
        assert_eq!(evaluate(&[Multiply, Plus], &[2, 3, 4]), Some(10));
        assert_eq!(evaluate(&[Minus, Minus], &[8, 2, 1]), Some(5));
        assert_eq!(evaluate(&[Divide, Divide], &[8, 4, 2]), Some(1));
    }

    #[wasm_bindgen_test]
    fn evaluate_rejects_division_with_remainder() {
        use Operator::*;
        assert_eq!(evaluate(&[Divide, Plus], &[7, 2, 1]), None);
        assert_eq!(evaluate(&[Plus, Divide], &[1, 7, 2]), None);
    }

    #[wasm_bindgen_test]
    fn generate() {
        // Generate 100 formulas and make sure the results are correct
        for _ in 0..100 {
            let challenge = generate_challenge().unwrap();
            let Challenge { formula, result } = serde_wasm_bindgen::from_value(challenge).unwrap();

            assert_eq!(formula.len(), 7, "Invalid length: {formula}");
            assert!((0..100).contains(&result), "Invalid result: {formula}");
            assert_eq!(
                Ok(Value::Int(i64::from(result))),
                eval(&formula),
                "Invalid result: {formula}"
            );
        }
    }
    // endregion
}
