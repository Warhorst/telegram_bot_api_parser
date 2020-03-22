use std::fmt::Debug;

/// Util to create parameterized tests. Creates a vec of input/expected pairs.
/// The input-values are processed by a certain closure, returning a result.
/// Each result will be compared to its expectation to assert equality.
pub trait TestCaseChecker {
    type Input: PartialEq + Debug;
    type Expected: PartialEq + Debug;

    /// Creates a Vec of input-values with their expected result, after a certain operation
    /// if run.
    fn create_test_cases(&self) -> Vec<(Self::Input, Self::Expected)>;

    /// Method to check if all expected result where created, after a given operation was run.
    ///
    /// The default implementation creates all test cases, run the given operation with
    ///the input value as parameter and calls assert_eq with the result and the expected value.
    fn assert_operation_returns_expectation(&self, operation: &dyn Fn(Self::Input) -> Self::Expected) {
        let test_cases = self.create_test_cases();

        test_cases.into_iter().for_each(
            |(input, expected)| assert_eq!(operation(input), expected)
        )
    }
}