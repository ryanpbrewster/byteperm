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

struct XorPermutation {
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
}
