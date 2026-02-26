fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn divide(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    println!("{}", add(2, 2));
    println!("{}", subtract(5, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6, 2), 3);
    }
}
