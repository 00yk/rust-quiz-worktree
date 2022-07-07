fn f<'a>()
where
    'a: 'a,
{
}
fn g<'a: 'a>() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz11() {
        let pf = f::<'static> as fn();
        let pg = g::<'static> as fn();
        print!("{}", pf == pg);
    }
}
