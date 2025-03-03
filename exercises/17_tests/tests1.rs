// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    use super::*;  // This line imports everything from the outer module, including `is_even`

    #[test]
    fn you_can_assert() {

        assert!(is_even(2), "2 should be even");
        assert!(!is_even(3), "3 should not be even");
        assert!(is_even(0), "0 should be even");  // Testing edge case
        assert!(!is_even(-1), "-1 should not be even");
        assert!(is_even(-2), "-2 should be even");
    }
}
