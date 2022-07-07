struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        print!("1");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz28() {
        let _guard = Guard;
        print!("3");
        let _ = Guard;
        print!("2");
    }
}
