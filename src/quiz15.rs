trait Trait {
    fn f(&self);
}

impl Trait for u32 {
    fn f(&self) {
        print!("1");
    }
}

impl<'a> Trait for &'a i32 {
    fn f(&self) {
        print!("2");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz15() {
        let x = &0;
        x.f();
    }
}
