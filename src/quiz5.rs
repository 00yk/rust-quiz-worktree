trait Trait {
    fn p(self);
}

impl<T> Trait for fn(T) {
    fn p(self) {
        print!("1");
    }
}

impl<T> Trait for fn(&T) {
    fn p(self) {
        print!("2");
    }
}

fn f(_: u8) {}
fn g(_: &u8) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz5() {
        let a: fn(_) = f;
        let b: fn(_) = g;
        let c: fn(&_) = g;
        a.p();
        b.p();
        c.p();
    }
}
