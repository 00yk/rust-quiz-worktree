use std::fmt::{self, Display};

struct S;

impl Display for S {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("1")
    }
}

impl Drop for S {
    fn drop(&mut self) {
        print!("2");
    }
}

fn f() -> S {
    S
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz25() {
        let S = f();
        print!("{}", S);
    }
    #[test]
    fn test_quiz25_stackoverflow() {

        A.m();                   // A::m()      , Self == @
        // without the Copy trait, (&A).m() would be a compilation error:
        // cannot move out of borrowed content
        // (&A).m();                // A::m()      , Self == *@
        (&&A).m();               // &&&A::m()   , Self == &@
        (&&&A).m();              // &&&A::m()   , Self == @
        A.refm();                // A::refm()   , Self == @
        (&A).refm();             // A::refm()   , Self == *@
        (&&A).refm();            // A::refm()   , Self == **@
        (&&&A).refm();           // &&&A::refm(), Self == @
    }
}

trait M { fn m(self); }
trait RefM { fn refm(&self); }
// #[derive(Clone, Copy)]
struct A;

impl M for    A { fn m(self) { println!("A::m()");    } }
impl M for &&&A { fn m(self) { println!("&&&A::m()"); } }

impl RefM for    A { fn refm(&self) { println!("A::refm()");    } }
impl RefM for &&&A { fn refm(&self) { println!("&&&A::refm()"); } }
