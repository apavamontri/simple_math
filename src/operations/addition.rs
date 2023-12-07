/// Adding two integers together
/// and return integer as a result
pub fn add(left: usize, right: usize) -> usize {
    private_add(left, right)
}

/// Perform actual addition operation
/// we want to use this to test the private function
/// with our unit testing
fn private_add(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_private_add() {
        let result = private_add(2, 2);
        assert_eq!(result, 4);
    }
}
