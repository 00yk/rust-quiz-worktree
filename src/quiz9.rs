macro_rules! m {
    (1) => {
        print!("1")
    };
    ($tt:tt) => {
        print!("2")
    };
}

macro_rules! e {
    ($e:expr) => {
        m!($e)
    };
}

macro_rules! t {
    ($tt:tt) => {
        e!($tt);
        m!($tt);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz9() {
        println!("quiz9");
        t!(1);
        println!();
        println!();
    }
}
