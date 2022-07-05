
macro_rules! m {
    ($($s:stmt)*) => {
        $(
            { stringify!($s); 1 }
        )<<*
    };
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz1() {
        // left associative
        println!("{}", 1 << 1 << 1 << 1);
        println!("{}", ((1 << 1) << 1) << 1);
        // right associative
        println!("{}", 1 << (1 << (1 << 1)));

        print!(
            "{}{}{}",
            m! { return || true },
            m! { (return) || true },
            m! { {return} || true },
        );
    }

}
