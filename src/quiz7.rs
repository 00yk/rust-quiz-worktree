#[repr(u8)]
enum Enum {
    First,
    Second,
}

impl Enum {
    fn p(self) {
        match self {
            First => print!("1"),
            Second => print!("2"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz7() {
        Enum::p(unsafe { std::mem::transmute(1u8) });
    }
}
