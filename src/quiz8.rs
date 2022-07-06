macro_rules! m {
    (==>) => { print!("1"); };
    (= = >) => { print!("2"); };
    (== >) => { print!("3"); };
    (= =>) => { print!("4"); };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz8() {
        println!("quiz8");
        m!(==>);
        m!(= = >);
        m!(== >);
        m!(= =>);
        println!("");
        println!("");

    }
}
