struct CreditCardNumber {
    number: String,
}

impl CreditCardNumber {
    fn new(number: &str) -> CreditCardNumber {
        CreditCardNumber {
            number: number.to_string(),
        }
    }

    fn is_valid(&self) -> bool {
        // TODO: Implement Luhn algorithm; not done here because not relevant for sample.
        true
    }
}

impl std::ops::Deref for CreditCardNumber {
    type Target = str;

    fn deref(&self) -> &str {
        &self.number
    }
}

fn main() {
    let card = &CreditCardNumber::new("4012888888881881");
    println!("The card number has a length of {} and is valid: {}", 
        // Note that len is a method on str, not on CreditCardNumber. Accessible because of Deref.
        card.len(),
        // Note that is_valid is a method on CreditCardNumber.
        card.is_valid());
}
