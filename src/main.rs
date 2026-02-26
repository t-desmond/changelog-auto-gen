mod math;
use math::arithmetics::MathOperations;

fn main() {
    let sum = MathOperations::add(5, 10);
    let difference = MathOperations::subtract(10, 5);
    let product = MathOperations::multiply(5, 10);
    let quotient = MathOperations::divide(10, 5);
    let modulus = MathOperations::modulus(10, 3);
    print!("Sum: {}\n", sum);
    print!("Difference: {}\n", difference);
    print!("Product: {}\n", product);
    print!("Quotient: {}\n", quotient);
    print!("Modulus: {}\n", modulus);
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

    #[test]
    fn test_modulus() {
        assert_eq!(MathOperations::modulus(10, 3), 1);
    }
}
