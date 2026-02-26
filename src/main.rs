fn main() {
    println!("{}", MathOperations::add(2, 2));
    println!("{}", MathOperations::subtract(5, 3));
    println!("{}", MathOperations::divide(6, 2));
    println!("{}", MathOperations::multiply(3, 4));
}

struct MathOperations;

impl MathOperations {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    fn divide(a: i32, b: i32) -> i32 {
        a / b
    }

    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(MathOperations::add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(MathOperations::subtract(5, 3), 2);
    }

    #[test]
    fn test_divide() {
        assert_eq!(MathOperations::divide(6, 2), 3);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(MathOperations::multiply(3, 4), 12);
    }
}
