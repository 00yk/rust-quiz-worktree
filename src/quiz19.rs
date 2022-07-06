struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz19() {
        let s = S;
        let _ = s;
        print!("2");
    }
}
