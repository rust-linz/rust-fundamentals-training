use std::ops::Add;

/// Recursively calculates the math pyramid.
///
/// # Arguments
///
/// * `numbers` - The numbers of the current level.
/// * `level` - The current level.
/// * `processor` - The function that is called for each level.
pub fn calculate_pyramid<T: Copy + Add<Output = T>, F: Fn(usize, &[T])>(
    numbers: &[T],
    level: usize,
    processor: &F,
) {
    // Allocate memory for the next level.
    let mut new_numbers = Vec::with_capacity(numbers.len() - 1);

    // Calculate the next level.
    for (a, b) in numbers.iter().zip(numbers.iter().skip(1)) {
        new_numbers.push(*a + *b);
    }

    // If we haven't reached the top of the pyramid, recursively calculate the next level.
    if numbers.len() > 1 {
        calculate_pyramid(&new_numbers, level + 1, processor);
    }

    // Process the current level (i.e. print it)
    processor(level, numbers);
}

#[cfg(test)]
mod tests {
     use super::*;

     #[test]
     fn test_calculate_pyramid() {
        let numbers = vec![1, 2, 3];
        let expected = vec![
            vec![1, 2, 3],
            vec![3, 5],
            vec![8],
        ];

        calculate_pyramid(&numbers, 0, &|level, numbers| {
            assert_eq!(numbers, &expected[level]);
        });
     }
}