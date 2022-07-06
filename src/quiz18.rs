struct S {
    f: fn(),
}

impl S {
    fn f(&self) {
        print!("1");
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz18() {
        println!("quiz 18");
        let print2 = || print!("2");
        S { f: print2 }.f();
        (S { f: print2 }.f)();
        println!();
        println!();
    }
}
