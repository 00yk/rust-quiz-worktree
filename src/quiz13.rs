struct S;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz13() {
        let [x, y] = &mut [S, S];
        let eq = x as *mut S == y as *mut S;
        println!("quiz13");
        print!("{}", eq as u8);
        println!();
        println!();
    }
}
