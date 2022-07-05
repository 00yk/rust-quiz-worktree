struct S;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz3() {
        let [x, y] = &mut [S, S];
        let eq = x as *mut S == y as *mut S;
        println!("quiz3");
        print!("{}", eq as u8);
        println!();
        println!();

    }
}
