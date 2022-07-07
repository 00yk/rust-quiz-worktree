trait Trait {
    fn f(&self);
    fn g(&self);
}

struct S;

impl S {
    fn f(&self) {
        print!("1");
    }

    fn g(&mut self) {
        print!("1");
    }
}

impl Trait for S {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz23() {
        fn f(&self) {
            print!("2");
        }

        fn g(&self) {
            print!("2");
        }
    }
}
