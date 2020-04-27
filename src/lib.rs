#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;

pub trait Permutation {
    fn apply(&self, input: u64) -> u64;
    fn unapply(&self, ouptut: u64) -> u64;
}

mod prime;

pub struct ShiftPermutation {
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

pub struct XorPermutation {
    key: u64,
}
impl Permutation for XorPermutation {
    fn apply(&self, input: u64) -> u64 {
        input ^ self.key
    }
    fn unapply(&self, output: u64) -> u64 {
        output ^ self.key
    }
}

pub struct MultiplicativePermutation {
  scalar: u64,
  inverse: u64,
}
impl MultiplicativePermutation {
  pub fn new(scalar: u64) -> MultiplicativePermutation {
    MultiplicativePermutation {
      scalar,
      inverse: prime::multiplicative_inverse(scalar),
    }
  }
}
impl Permutation for MultiplicativePermutation {
  fn apply(&self, input: u64) -> u64 {
    self.scalar.wrapping_mul(input)
  }
  fn unapply(&self, output: u64) -> u64 {
    self.inverse.wrapping_mul(output)
  }
}


#[cfg(test)]
mod test {
    use super::*;

    #[quickcheck]
    fn shift_invertible(amount: u64, input: u64) {
        let perm = ShiftPermutation { amount };
        assert_eq!(
            perm.unapply(perm.apply(input)),
            input,
            "shift({}) not invertible on {}",
            amount,
            input
        );
    }

    #[quickcheck]
    fn xor_invertible(key: u64, input: u64) {
        let perm = XorPermutation { key };
        assert_eq!(
            perm.unapply(perm.apply(input)),
            input,
            "xor({}) not invertible on {}",
            key,
            input
        );
    }

    #[quickcheck]
    fn multiply_invertible(key: u64, input: u64) {
        let perm = MultiplicativePermutation::new(2 * key + 1);
        assert_eq!(
            perm.unapply(perm.apply(input)),
            input,
            "mult({}) not invertible on {}",
            key,
            input
        );
    }

    #[test]
    fn shift_wrap() {
        let amount = std::u64::MAX;
        let perm = ShiftPermutation { amount };
        let input = 42;
        assert_eq!(
            perm.unapply(perm.apply(input)),
            input,
            "shift({}) not invertible on {}",
            amount,
            input
        );
    }
    #[test]
    fn shift_sequence() {
        // ShiftPermutation kind of sucks, because adjacent inputs produce adjacent outputs.
        let perm = ShiftPermutation { amount: 12345 };
        assert_eq!(
            perm.apply(42) + 1,
            perm.apply(43),
        );
    }

    #[test]
    fn xor_sequence() {
        // XorPermutation is a bit better, but still not great.
        // Inputs that differ by a single bit will produce outputs that differ by a single bit.
        let perm = XorPermutation { key: 123_456_789 };
        assert_eq!(
            perm.apply(42),
            perm.apply(43) ^ 1,
        );
    }

    #[test]
    fn mul_zero() {
        // MultiplicativePermutation is better still, but it always maps 0 to 0, and 1 to the key.
        let perm = MultiplicativePermutation::new(123_456_789);
        assert_eq!(perm.apply(0), 0);
        assert_eq!(perm.apply(1), 123_456_789);
    }
}
