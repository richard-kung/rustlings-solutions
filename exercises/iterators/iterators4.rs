// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.


pub fn factorial(num: u64) -> u64 {
    // === Method 1 : Recursion ===
    //if (num > 1) {
    //    num * factorial(num - 1)
    //} else {
    //    1
    //}

    // === Method 2 : Range ===
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
