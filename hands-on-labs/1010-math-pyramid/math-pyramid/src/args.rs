// ================================================================================
// LEARNINGS:
// - How to access command line arguments in Rust
//   (without helper crates like [clap](https://docs.rs/clap/latest/clap/))
// - Working with recoverable errors (`Result`)
// - Writing generic methods, using trait bounds
// - Working with iterators
// - `match` expressions
// - Writing basic unit tests
// ================================================================================

/// Reads the base length of the math pyramid from the command line arguments.
///
/// Note that if the base length is not specified in the command line arguments,
/// *5* is used as the default length of the base.
///
/// # Arguments
///
/// * `args` - The command line arguments as an iterator. Accepting any iterator
///   instead of [`std::env::Args`] allows us to test the function without having
///   to pass in the actual command line arguments.
///
/// # Returns
///
/// * `Ok(usize)` - The base length of the math pyramid.
/// * `Err(String)` - The error message.
pub fn get_base_length<T>(mut args: T) -> Result<usize, String>
where
    T: Iterator<Item = String>,
{
    // Skip the first argument, which is the name of the program.
    // Use 5 as the default base length.
    let a = args.nth(1).unwrap_or("5".into());

    // Parse the base length from the command line arguments.
    match a.parse() {
        Ok(n) if !matches!(n, 2..=10) => {
            Err(format!("Base length must be between 2 and 10, but was {n}"))
        }
        Ok(n) => Ok(n),
        Err(_) => Err(format!(
            r#"Invalid base length. "{a}" is not a valid number."#
        )),
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_get_base_length() {
        let args = vec!["".into(), "5".into()].into_iter();
        assert_eq!(get_base_length(args), Ok(5));
    }

    #[test]
    fn test_get_base_length_default() {
        let args = vec![String::new()].into_iter();
        assert_eq!(get_base_length(args), Ok(5));
    }

    #[test]
    fn test_get_base_length_error() {
        let args = vec!["".into(), "abc".into()].into_iter();
        assert!(get_base_length(args).is_err());
    }

    #[rstest]
    #[case(1)]
    #[case(11)]
    fn test_get_base_length_out_of_bounds(#[case] length: usize) {
        let args = vec!["".into(), length.to_string()].into_iter();
        assert!(get_base_length(args).is_err());
    }
}
