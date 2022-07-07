struct D(u8);

impl Drop for D {
    fn drop(&mut self) {
        print!("{}", self.0);
    }
}

struct S {
    d: D,
    x: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz12() {
        let S { x, .. } = S { d: D(1), x: 2 };
        print!("{}", x);

        let S { ref x, .. } = S { d: D(3), x: 4 };
        print!("{}", x);
    }
}
