struct S {
    x: i32,
}

const S: S = S { x: 2 };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz3() {
        let v = &mut S;
        v.x += 1;
        S.x += 1;
        print!("{}{}", v.x, S.x);
    }
}
