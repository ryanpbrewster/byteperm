/// Compute the multiplicative inverse of `a` mod 2^64.
///     a.wrapping_mul(multiplicative_inverse(a)) == 1
///
/// Source: https://cp-algorithms.com/algebra/montgomery_multiplication.html#toc-tgt-2
/// Rough description:
///     a*x == 1 (mod 2^k)
/// implies that
///     a*x*(2-a*x) == 1 (mod 2^(2k))
/// For any odd value of a,
///     a == 1 (mod 2)
/// so start with x=1, k=1, then repeatedly iterate this x = x * (2 - a * x) loop.
pub fn multiplicative_inverse(a: u64) -> u64 {
  assert!(a % 2 == 1);
  let mut x: u64 = 1; // a * x == 1 (mod 2^1)
  // Each iteration doubles the exponent: 2^2, 2^4, 2^8, 2^16, 2^32, 2^64
  for _ in 0 .. 7 {
    x = x.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x)));
  }
  x
}

#[cfg(test)]
mod test {
  use super::*;
  use quickcheck::*;

  #[test]
  fn multiplicative_inverse_smoke_test() {
    let a = 12345;
    let r = multiplicative_inverse(a);
    assert_eq!(r, 5288216061308878345);
    assert_eq!(a.wrapping_mul(r), 1);
  }

  #[test]
  fn multiplicative_inverse_examples_test() {
    assert_eq!(multiplicative_inverse(1), 1);
    assert_eq!(multiplicative_inverse(3), 12297829382473034411);
    assert_eq!(multiplicative_inverse(19), 9708812670373448219);
    assert_eq!(multiplicative_inverse(123_456_789), 16969517551553616445);
    assert_eq!(multiplicative_inverse(std::u64::MAX), 18446744073709551615);
  }

  #[quickcheck]
  fn multiplicative_inverse_check(a0: u64) {
    let a = 2 * a0 + 1; // ensure it's odd
    assert_eq!(a.wrapping_mul(multiplicative_inverse(a)), 1);
  }
}
