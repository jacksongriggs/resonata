#[cfg(test)]
mod tests {
    use crate::notes::Accidental;

    #[test]
    fn test_add() {
        let a = Accidental::Flat(1);
        let b = Accidental::Sharp(2);
        assert_eq!(a + 3, b);
    }
}
