#![no_std]

/// Iterates over all `bits` width numbers in hamming weight order and
/// sub-ordered by numerical order. Asserts that `bits < 64`.
#[inline]
pub fn hamming_iter(bits: u32) -> impl Iterator<Item = u64> {
    assert!(bits <= 64);
    if bits < 64 {
        either::Left(
            (0..=bits)
                .flat_map(move |i| hamming_weight_iter(i).take_while(move |&n| n < 1 << bits)),
        )
    } else {
        either::Right((0..=64).flat_map(hamming_weight_iter))
    }
}

/// Iterate over all 64-bit integers with
#[inline]
pub fn hamming_weight_iter(weight: u32) -> impl Iterator<Item = u64> {
    if weight < 2 {
        either::Left(if weight == 0 {
            either::Left(core::iter::once(0))
        } else {
            either::Right((0..64).map(|b| 1 << b))
        })
    } else {
        // Find number of numbers with this distance.
        let combinations = (64 - weight as usize + 1..=64)
            .try_fold(1usize, |acc, n| acc.checked_mul(n))
            .and_then(|n| {
                (2..=weight as usize)
                    .try_fold(1usize, |acc, n| acc.checked_mul(n))
                    .map(|m| n / m)
            });
        either::Right(if let Some(combinations) = combinations {
            either::Left(hamming_ascend((1 << weight) - 1).take(combinations))
        } else {
            either::Right(hamming_ascend((1 << weight) - 1))
        })
    }
}

/// Iterates over ascending equal hamming weight numbers starting at `start`.
fn hamming_ascend(start: u64) -> impl Iterator<Item = u64> {
    use core::num::Wrapping;
    let mut acc = Wrapping(start);
    core::iter::repeat(()).map(move |_| {
        let res = acc.0;
        let c = acc & ((!acc) + Wrapping(1));
        let r = acc + c;
        acc = (((r ^ acc) >> 2) / c) | r;
        res
    })
}
