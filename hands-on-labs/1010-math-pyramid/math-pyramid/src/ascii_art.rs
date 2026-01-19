// ================================================================================
// LEARNINGS:
// - Working with constants and constant functions
// - Simple enums
// - Using `match` expressions
// - `char` vs. `byte` literals
// - Working with iterators
// - `for` loops
// - Building strings
// - `if let` expressions
// - Number formatting with `format!` and `println!`
// ================================================================================

// Some constants for formatting our math pyramid
pub const TOP_DECORATOR: &str = "┌──────┐";
pub const BOTTOM_DECORATOR: &str = "└──────┘";
pub const SEPARATOR_LENGTH: usize = 3;
pub const SEPARATOR_CHAR: u8 = b' '; // Note that this is a byte, not a character
pub const BAR: char = '│';

/// Helper struct for repeating a char in const fn.
struct Buf<const N: usize>([u8; N]);

/// Repeats a given single-byte-char `c` `N` times.
const fn repeat_impl<const N: usize>(c: u8) -> Buf<N> {
    let mut buffer = [0; N];
    let mut i = 0;

    while i < N {
        buffer[i] = c;
        i += 1;
    }

    Buf(buffer)
}

/// Repeats a given single-byte-char `c` `n` times.
macro_rules! repeat {
    ($c:expr, $n:expr) => {{
        const CONCAT_BUF: Buf<$n> = repeat_impl($c);
        unsafe { core::str::from_utf8_unchecked(&CONCAT_BUF.0) }
    }};
}

pub const SEPARATOR: &str = repeat!(SEPARATOR_CHAR, SEPARATOR_LENGTH);
pub const INDENTATION_LEN: usize = (TOP_DECORATOR.len() / BAR.len_utf8() + SEPARATOR_LENGTH) / 2;
pub const INDENTATION: &str = repeat!(SEPARATOR_CHAR, INDENTATION_LEN);

pub enum Decorator {
    Top,
    Bottom,
}

impl Decorator {
    /// Returns the decorator string for the given decorator.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Decorator::Top => TOP_DECORATOR,
            Decorator::Bottom => BOTTOM_DECORATOR,
        }
    }
}

/// Returns a string of `n` copies of `decorator` with separating spaces between.
///
/// Note that `repeated_decorator` and `repeated_decorator_iter` serve the same purpose.
/// They are just different implementations demonstrating different solution approaches.
///
/// # Arguments
///
/// * `decorator` - The decorator to repeat.
/// * `n` - The number of times to repeat the decorator.
pub fn repeated_decorator(decorator: Decorator, n: usize) -> String {
    // Get the decorator string
    let decorator_str = decorator.as_str();

    // Create string that will receive the repeated string. Note that
    // `decorator_str.len()` returns the number of bytes, not the number of characters.
    // Additionally, it is a const function, so it is evaluated at compile time.
    let mut result =
        String::with_capacity(n * (decorator_str.len() + SEPARATOR_LENGTH) - SEPARATOR_LENGTH);

    // Add the copies of the string with separating spaces
    for _ in 0..n - 1 {
        result.push_str(decorator_str);
        result.push_str(SEPARATOR);
    }
    result.push_str(decorator_str);

    result
}

/// Returns an iterator over `n` copies of `decorator` with separating spaces between.
///
/// Note that `repeated_decorator` and `repeated_decorator_iter` serve the same purpose.
/// They are just different implementations demonstrating different solution approaches.
///
/// # Arguments
///
/// * `decorator` - The decorator to repeat.
/// * `n` - The number of times to repeat the decorator.
pub fn repeated_decorator_iter(
    decorator: Decorator,
    n: usize,
) -> impl Iterator<Item = &'static str> {
    (0..2 * n - 1).map(move |i| {
        if i % 2 == 0 {
            decorator.as_str()
        } else {
            SEPARATOR
        }
    })
}

/// Returns a string with the given numbers between bars and separating spaces between them.
/// 
/// # Arguments
/// 
/// * `numbers` - The numbers to format
pub fn repeated_numbers_iter(numbers: impl Iterator<Item = u16>) -> String {
    let mut result;
    if let Some(size) = numbers.size_hint().1 {
        // Note how we use `size_hint` to pre-allocate the string.
        result = String::with_capacity(
            (BAR.len_utf8() * 2 + TOP_DECORATOR.len() - 2) * size + SEPARATOR_LENGTH * (size - 1),
        );
    } else {
        // If we don't know the size, we just use the default capacity.
        result = String::new();
    }

    // Add the copies of the string with separating spaces. Separating spaces
    // are ommitted before the first number.
    let mut first = true;
    numbers.map(number_between_bars).for_each(|s| {
        if first {
            first = false;
        } else {
            result.push_str(SEPARATOR);
        }
        result.push_str(&s);
    });

    result
}

/// Returns a string containing the given number enclosed in bars (see `BAR` character).
/// 
/// Example: `number_between_bars(42)` returns `"│   42 │"`.
/// 
/// # Arguments
/// 
/// * `n` - The number to enclose in bars.
pub fn number_between_bars(n: u16) -> String {
    format!("│ {n:>4} │")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_repeat_macro() {
        assert_eq!(repeat!(b' ', 3), "   ");
    }

    #[rstest]
    #[case(Decorator::Top, 3, format!("{0}{1}{0}{1}{0}", TOP_DECORATOR, SEPARATOR))]
    #[case(Decorator::Top, 1, TOP_DECORATOR.into())]
    fn test_repeat_str(#[case] s: Decorator, #[case] n: usize, #[case] expected: String) {
        assert_eq!(repeated_decorator(s, n), expected);
    }

    #[rstest]
    #[case(Decorator::Top, 3, vec![TOP_DECORATOR, SEPARATOR, TOP_DECORATOR, SEPARATOR, TOP_DECORATOR])]
    #[case(Decorator::Top, 1, vec![TOP_DECORATOR])]
    fn test_repeat_iter(#[case] s: Decorator, #[case] n: usize, #[case] expected: Vec<&str>) {
        assert!(repeated_decorator_iter(s, n).eq(expected));
    }

    #[test]
    fn test_number_between_bars() {
        assert_eq!(number_between_bars(42), "│   42 │");
    }

    #[rstest]
    #[case(vec![1, 2, 3], format!("{1}{0}{2}{0}{3}", SEPARATOR, number_between_bars(1), number_between_bars(2), number_between_bars(3)))]
    #[case(vec![1], number_between_bars(1))]
    fn test_repeated_numbers_iter(#[case] s: Vec<u16>, #[case] expected: String) {
        assert_eq!(repeated_numbers_iter(s.into_iter()), expected);
    }
}
