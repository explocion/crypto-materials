use rug::Integer;

// requires a, b positive
// returns (gcd, s, t) such that as + bt = gcd
pub fn extended_gcd(a: Integer, b: Integer) -> (Integer, Integer, Integer) {
    let mut r = (b, a);
    let mut s = (Integer::from(0), Integer::from(1));
    let mut t = (Integer::from(1), Integer::from(0));
    while r.0 != 0 {
        let (q, rem) = r.1.div_rem(r.0.clone());
        r = (rem, r.0);
        s = (s.1 - &q * &s.0, s.0);
        t = (t.1 - &q * &t.0, t.0);
    }
    (r.1, s.1, t.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rug::rand::RandState;

    #[test]
    fn random_test() {
        let mut rng = RandState::new();
        let a = Integer::from(Integer::random_bits(4096, &mut rng));
        let b = Integer::from(Integer::random_bits(4096, &mut rng));
        assert_eq!(
            extended_gcd(a.clone(), b.clone()),
            a.extended_gcd(b, Integer::new())
        );
    }
}
