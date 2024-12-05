use crate::core::fields::{ExtensionOf, Field};

/// Folds values recursively in `O(n)` by a hierarchical application of folding factors.
///
/// i.e. folding `n = 8` values with `folding_factors = [x, y, z]`:
///
/// ```text
///               n2=n1+x*n2
///           /               \
///     n1=n3+y*n4          n2=n5+y*n6
///      /      \            /      \
/// n3=a+z*b  n4=c+z*d  n5=e+z*f  n6=g+z*h
///   /  \      /  \      /  \      /  \
///  a    b    c    d    e    f    g    h
/// ```
///
/// # Panics
///
/// Panics if the number of values is not a power of two or if an incorrect number of of folding
/// factors is provided.
// TODO(Andrew): Can be made to run >10x faster by unrolling lower layers of recursion
pub fn fold<F: Field, E: ExtensionOf<F>>(values: &[F], folding_factors: &[E]) -> E {
    let n = values.len();
    assert_eq!(n, 1 << folding_factors.len());
    if n == 1 {
        return values[0].into();
    }
    let (lhs_values, rhs_values) = values.split_at(n / 2);
    let (folding_factor, folding_factors) = folding_factors.split_first().unwrap();
    let lhs_val = fold(lhs_values, folding_factors);
    let rhs_val = fold(rhs_values, folding_factors);
    lhs_val + rhs_val * *folding_factor
}

pub fn fold_gpu<F: Field, E: ExtensionOf<F>>(values: &[F], folding_factors: &[E]) -> E {
    let n = values.len();
    assert_eq!(n, 1 << folding_factors.len());
    if n == 1 {
        return values[0].into();
    }
    let (lhs_values, rhs_values) = values.split_at(n / 2);
    let (folding_factor, folding_factors) = folding_factors.split_first().unwrap();
    let lhs_val = fold(lhs_values, folding_factors);
    let rhs_val = fold(rhs_values, folding_factors);
    lhs_val + rhs_val * *folding_factor
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::fields::m31::M31;
    use crate::core::fields::qm31::QM31;
    #[test]
    fn test_fold_works() {
        // Example input: power-of-two values and appropriate folding factors
        let values = vec![
            M31(1),
            M31(2),
            M31(3),
            M31(4),
            M31(5),
            M31(6),
            M31(7),
            M31(8),
        ];
        let folding_factors = vec![
            QM31::from_u32_unchecked(2, 0, 0, 0),
            QM31::from_u32_unchecked(3, 0, 0, 0),
            QM31::from_u32_unchecked(4, 0, 0, 0),
        ];
        let result = fold(&values, &folding_factors);

        // Replace with the expected result based on the function's logic
        let expected = QM31::from_u32_unchecked(358, 0, 0, 0);
        assert_eq!(result, expected, "The fold_recursive result is incorrect");
    }
}
