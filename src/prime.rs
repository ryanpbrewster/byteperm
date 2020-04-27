use std::convert::TryFrom;

fn multiplicative_inverse(a: u64) -> u64 {
  assert!(a % 2 == 1);
  let  x1: u64 = 1; // a' (mod 2^1)
  let  x2: u64 =  x1.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul( x1))); // a' (mod 2^2)
  let  x4: u64 =  x2.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul( x2))); // a' (mod 2^4)
  let  x8: u64 =  x4.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul( x4))); // a' (mod 2^8)
  let x16: u64 =  x8.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul( x8))); // a' (mod 2^16)
  let x32: u64 = x16.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x16))); // a' (mod 2^32)
  let x64: u64 = x32.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x32))); // a' (mod 2^64)
  return x64;
}

#[cfg(test)]
mod test {
  use super::*;
  use quickcheck::*;

  #[test]
  fn multiplicative_inverse_smoke_test() {
    let a = 12345;
    let r = multiplicative_inverse(a);
    assert_eq!(a.wrapping_mul(r), 1);
  }

  #[quickcheck]
  fn multiplicative_inverse_check(a0: u64) {
    let a = 2 * a0 + 1; // ensure it's odd
    assert_eq!(a.wrapping_mul(multiplicative_inverse(a)), 1);
  }
}
