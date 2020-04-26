#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;

trait Permutation {
  fn apply(&self, input: u64) -> u64;
  fn unapply(&self, ouptut: u64) -> u64;
}

struct ShiftPermutation {
  amount: u64,
}
impl Permutation for ShiftPermutation {
  fn apply(&self, input: u64) -> u64 {
    input.wrapping_add(self.amount)
  }
  fn unapply(&self, output: u64) -> u64 {
    output.wrapping_sub(self.amount)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[quickcheck]
  fn shift_invertible(amount: u64, input: u64) {
    let perm = ShiftPermutation { amount };
    assert_eq!(perm.unapply(perm.apply(input)), input);
  }

  #[test]
  fn shift_wrap() {
    let perm = ShiftPermutation { amount: std::u64::MAX };
    assert_eq!(perm.unapply(perm.apply(42)), 42);
  }
}
