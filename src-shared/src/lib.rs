pub mod handlers;

use rand::random;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn rand() -> u8 {
    random()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_rand() {
        let result = rand();
        assert!(result < u8::MIN);
        assert!(result > u8::MAX);
    }
}
