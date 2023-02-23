struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("the number is too big or too small")
        }
        Guess { value }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);

        assert_eq!(
            result, 4,
            "Something is wrong, the number is {} and {}",
            2, result
        );
    }
    #[test]
    #[should_panic]
    fn is_valid_guess() {
        Guess::new(10);
    }
}
