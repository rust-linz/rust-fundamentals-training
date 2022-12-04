use std::env::Args;

/// Reads the base length of the math pyramid from the command line arguments.
/// 
/// Note that if the base length is not specified in the command line arguments,
/// *5* is used as the default length of the base.
///
/// # Arguments
/// 
/// * `args` - The command line arguments as an iterator.
/// 
/// # Returns
/// 
/// * [Ok(u32)] - The base length of the math pyramid.
/// * [Err(String)] - The error message.
pub fn get_base_length(args: Args) -> Result<usize, String>
{
    get_base_length_impl(args)
}

/// Internal implementation of `get_base_length` for testing purposes.
/// 
/// This version of the function takes an iterator instead of [std::env::Args].
/// This allows us to test the function without having to pass in the actual
/// command line arguments.
fn get_base_length_impl<T>(mut args: T) -> Result<usize, String>
where
    T: Iterator<Item = String>,
{
    // Skip the first argument, which is the name of the program.
    // Use 5 as the default base length.
    let a = args.nth(1).unwrap_or("5".into());

    // Parse the base length from the command line arguments.
    match a.parse() {
        Ok(n) => if !matches!(n, 2..=10) {
            Err(format!("Base length must be between 2 and 10, but was {n}"))
            } else {
                Ok(n)
            },
        Err(_) => Err(format!(r#"Invalid base length. "{a}" is not a valid number."#)),
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_get_base_length() {
        let args = vec!["".into(), "5".into()].into_iter();
        assert_eq!(get_base_length_impl(args), Ok(5));
    }

    #[test]
    fn test_get_base_length_error() {
        let args = vec!["".into(), "abc".into()].into_iter();
        assert!(get_base_length_impl(args).is_err());
    }

    #[rstest]
    #[case(1)]
    #[case(11)]
    fn test_get_base_length_out_of_bounds(#[case] length: usize) {
        let args = vec!["".into(), length.to_string()].into_iter();
        assert!(get_base_length_impl(args).is_err());
    }
}
