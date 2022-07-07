#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz16() {
        let mut x = 4;
        --x;
        print!("{}{}", --x, --x);
    }
}
