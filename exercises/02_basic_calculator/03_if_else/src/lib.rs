/// Return `true` if `n` is even, `false` otherwise.
fn is_even(n: u32) -> bool {
    let remainder:u32 = n % 2;
    if remainder > 0{
        false
    }
    else{
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::is_even;

    #[test]
    fn one() {
        assert!(!is_even(1));
    }

    #[test]
    fn two() {
        assert!(is_even(2));
    }

    #[test]
    fn high() {
        assert!(!is_even(231));
    }
}
