
fn check(x: i32) -> bool {
    print!("{}", x);
    false
}

fn check1(x: i32) -> bool {
    print!("{}", x);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz32() {
        match (1, 2) {
            (x, _) | (_, x) if check(x) => {
                print!("3")
            }
            _ => print!("4"),
        }
        match (1, 2) {
            (x, _) | (_, x) if check1(x) => {
                print!("3")
            }
            _ => print!("4"),
        }
    }
}
