

use std::{hash::Hasher, time::Instant};

use core::num::NonZero;

const MULTIPLIER: u64 = 6364136223846793005;
const INCREMENT: u64 = 1442695040888963407;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RNG {
    pub state: u64
}


impl RNG {
    #[inline]
    #[must_use]
    /// Creates a new random number generator with the specified seed.
    pub fn new(seed: u64) -> RNG {
        Self {state: seed}
    }

    /// Creates a new random number generator with an arbitrary seed.
    #[must_use]
    pub fn new_entropic() -> RNG {
        let seed_seed = Instant::now();
        // SAFETY: the copy ensures no writes into unowned memory.
        // and there no fucks to give about what values are actually
        // in the bytes. that's kinda the whole point.
        #[expect(clippy::clone_on_copy)]
        let seed_seed_bytes = unsafe {
            core::mem::transmute::<Instant, [u8; 16]>(seed_seed)
        }.clone();
        let mut hasher = std::hash::DefaultHasher::new();
        hasher.write(&seed_seed_bytes);
        Self::new(hasher.finish())
    }

    #[must_use]
    pub const fn next_bool(&mut self) -> bool {
        self.next_u32() & 1 == 0
    }

    #[inline]
    #[must_use]
    pub const fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        let count = (self.state >> 59) as u32;

        self.state = x.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        x ^= x >> 18;
        ((x >> 27) as u32).rotate_right(count)
    }

    #[inline]
    #[must_use]
    pub const fn next_nonzero_u32(&mut self) -> NonZero<u32> {
        loop {
            if let Some(ans) = NonZero::new(self.next_u32()) {
                return ans
            }
        }
    }

    #[inline]
    #[must_use]
    pub const fn next_u64(&mut self) -> u64 {
        ((self.next_u32() as u64) << 32) + self.next_u32() as u64
    }

    #[inline]
    #[must_use]
    #[expect(dead_code)]
    pub const fn next_nonzero_u64(&mut self) -> NonZero<u64> {
        loop {
            if let Some(ans) = NonZero::new(self.next_u64()) {
                return ans
            }
        }
    }

    #[inline]
    #[must_use]
    pub const fn next_u128(&mut self) -> u128 {
        // replace with disjoint xor once stabilized
        ((self.next_u64() as u128) << 64) + self.next_u64() as u128
    }

    #[inline]
    #[must_use]
    pub const fn next_nonzero_u128(&mut self) -> NonZero<u128> {
        loop {
            if let Some(ans) = NonZero::new(self.next_u128()) {
                return ans
            }
        }
    }

    #[must_use]
    #[expect(dead_code)]
    pub fn with_advantage<V, G>(&mut self, rerolls: u32, mut generator: G) -> V 
    where V: Ord, G: FnMut(&mut RNG) -> V {
        let mut cur = generator(self);
        for _ in 0..rerolls {
            cur = cur.max(generator(self))
        }
        cur
    }

    #[must_use]
    #[expect(dead_code)]
    pub fn with_disadvantage<V, G>(&mut self, rerolls: u32, mut generator: G) -> V
    where V: Ord, G: FnMut(&mut RNG) -> V {
        let mut cur = generator(self);
        for _ in 0..rerolls {
            cur = cur.min(generator(self))
        }
        cur
    }
    
    #[inline]
    #[must_use]
    pub fn next_that_is<V, G, C>(&mut self, mut generator: G, mut criteria: C) -> V
    where G: FnMut(&mut RNG) -> V, C: FnMut(&V) -> bool {
        loop {
            let ans = generator(self);
            if criteria(&ans) {
                return ans
            }
        }
    }
}
