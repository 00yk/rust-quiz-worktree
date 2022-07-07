#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz24() {
        let x: u8 = 1;
        const K: u8 = 2;

        macro_rules! m {
            () => {
                print!("{}{}", x, K);
            };
        }

        {
            let x: u8 = 3;
            const K: u8 = 4;

            m!();
        }
    }
}
