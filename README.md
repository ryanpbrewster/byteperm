# Permutations of 64 bit integers

Suppose you have a bunch of sequential 64-bit integers, and you want to
(reversibly) map them to "random"-looking integers. Let's call the mapping
function `M`, with inverse `M'`.

Because we want the mapping to be reversible, every input must map to a unique
output. The mapping must be bijective. Bijective functions on 64-bit integers
can be thought of as permutations of the entire 64-bit integer space. But
representing an arbitrary permutation of 2^64 integers might be very expensive.

This repository is an attempt to come up with space- and computation-efficient
representations of permutations over 64-bit integers.

### Very simple permutations

The simplest permutation is the identity function.
```
M(n) = n
M'(r) = r
```

Example: `[0, 1, 2, 3, 4] -> [0, 1, 2, 3, 4]`

A slightly less trivial permutation is a simple "shift" permutation, where `n`
maps to `n+x` for some offset `x`.
```
M(n)  = n + x [mod 2^64]
M'(r) = r - x [mod 2^64]
```
and here you have to be careful to deal with integer overflow.

Example: `[0, 1, 2, 3, 4] -> [42, 43, 44, 45, 46]`

### Affine permutations

If a "shift" permutation is an addition, we can also look at multiplication.
```
M(n)  = a  * n [mod 2^64]
M'(r) = a' * r [mod 2^64]
```
for some scalar `a`. In order to be invertible (i.e., for `a'` to exist), `a`
must be coprime to 2^64, aka `gcd(a, 2^64) = 1`.

Even when we know that `gcd(a, 2^64) = 1`, I found it a bit tricky to compute
the multiplicative inverse, `a'`. You can normally use the extended Euclidean
algorithm, but the standard implementation uses signed integers and dealing
with the edge cases around unsigned and signed 64 bit integers gave me a
headache. Luckily, cryptographers have come up with clever algorithms
specifically for finding multiplicative inverses for powers of 2. (Reference:
[Montgomery Multiplication](https://cp-algorithms.com/algebra/montgomery_multiplication.html#toc-tgt-2).

Without proof, consider that
```
a*x == 1 [mod 2^k]
```
implies that
```
a*x*(2-a*x) == 1 [mod 2^2k]
```

Any odd integer `a` will satisfy
```
a === 1 [mod 2]
```
so we can start with `x = 1` (for `k=1`) and work upwards
```
x_1 = 1
x_2 = x_1 * (2 - a * x_1)
...
x_64 = x_32 * (2 - a * x_32)
```
where you again have to be careful of integer overflow.

With this in hand, we can generate an invertible mapping using any odd integer
`a` and it's multiplicative inverse, `a'`.

Example: `M(n) = 123_456_789 * n [mod 2^64]`, `M'(n) = 16969517551553616445 * n [mod 2^64]`
`[0, 1, 2, 3, 4]` -> [0, 123456789, 246913578, 370370367, 493827156]`

We can combine multiplication and addition.
```
M(n)  = a * n + b [mod 2^64]
M'(r) = a' * (r - b) [mod 2^64]
```
for any odd value `a`.

### Block Ciphers

Block ciphers, like [Blowfish](https://en.wikipedia.org/wiki/Blowfish_(cipher)),
[Skipjack](https://en.wikipedia.org/wiki/Skipjack_(cipher)),
and [Rijndael (aka AES)](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard),
are invertible mappings on fixed-size blocks.

Further reading
[here](http://www.adammil.net/blog/v102_Permuting_Integers_and_Creating_Bijective_Functions_using_Block_Ciphers.html).
