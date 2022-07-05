trait Trait {
    fn f(&self);
}

impl<F: FnOnce() -> bool> Trait for F {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for () {
    fn f(&self) {
        print!("2");
    }
}

fn f() -> () {
    (return) || true;
}

////
// the following example doesn't compile
// fn f() -> () {
//     (return) || true
// }
////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz2() {
        let x = || { (return) || true; };
        x().f();

        let x = loop { (break) || true; };
        x.f();

        let x = || { return (|| true); };
        x().f();

        let x = loop { break (|| true); };
        x.f();

        let x = || { return || true; };
        x().f();

        let x = loop { break || true; };
        x.f();
    }
}
