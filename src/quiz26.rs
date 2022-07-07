#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz26() {
        let input = vec![1, 2, 3];

        let parity = input.iter().map(|x| {
            print!("{}", x);
            x % 2
        });

        for p in parity {
            print!("{}", p);
        }
    }
}
