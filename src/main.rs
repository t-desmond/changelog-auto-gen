mod math;
use math::arithmetics::MathOperations;
use math::geometry::Point;

fn main() {
    let point1 = Point::new(3.0, 4.0);
    let point2 = Point::new(7.0, 1.0);
    print!("Distance between points: {}\n", point1.distance(&point2));

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

    #[test]
    fn test_distance() {
        let point1 = Point::new(0.0, 0.0);
        let point2 = Point::new(3.0, 4.0);
        assert_eq!(point1.distance(&point2), 5.0);
    }
}
