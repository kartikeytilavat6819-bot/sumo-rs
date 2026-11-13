//! Port of utils/common/RandHelper.{h,cpp}. SumoRNG is std::mt19937; ported by hand so
//! random sequences are bit-identical to the C++ build.
use std::cell::RefCell;

use crate::options::option::SumoOption;
use crate::options::options_cont::OptionsCont;

pub fn splitmix64(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9e3779b97f4a7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

/// std::mt19937
#[derive(Clone)]
pub struct Mt19937 {
    mt: [u32; 624],
    idx: usize,
}

impl Mt19937 {
    pub const DEFAULT_SEED: u32 = 5489;
    pub fn new(seed: u32) -> Self {
        let mut r = Mt19937 { mt: [0; 624], idx: 624 };
        r.seed(seed);
        r
    }
    pub fn seed(&mut self, seed: u32) {
        self.mt[0] = seed;
        for i in 1..624 {
            let prev = self.mt[i - 1];
            self.mt[i] = 1812433253u32.wrapping_mul(prev ^ (prev >> 30)).wrapping_add(i as u32);
        }
        self.idx = 624;
    }
    fn twist(&mut self) {
        for i in 0..624 {
            let y = (self.mt[i] & 0x8000_0000) | (self.mt[(i + 1) % 624] & 0x7fff_ffff);
            let mut v = self.mt[(i + 397) % 624] ^ (y >> 1);
            if y & 1 != 0 {
                v ^= 0x9908_b0df;
            }
            self.mt[i] = v;
        }
        self.idx = 0;
    }
    pub fn next_u32(&mut self) -> u32 {
        if self.idx >= 624 {
            self.twist();
        }
        let mut y = self.mt[self.idx];
        self.idx += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^ (y >> 18)
    }
    pub fn discard(&mut self, n: u64) {
        for _ in 0..n {
            self.next_u32();
        }
    }
}

#[derive(Clone)]
pub struct SumoRNG {
    engine: Mt19937,
    pub count: u64,
    pub orig_seed: i32,
    pub id: String,
}

impl SumoRNG {
    pub fn new(id: &str) -> Self {
        SumoRNG { engine: Mt19937::new(Mt19937::DEFAULT_SEED), count: 0, orig_seed: Mt19937::DEFAULT_SEED as i32, id: id.into() }
    }
    pub fn set_seed(&mut self, seed: i32) {
        self.orig_seed = seed;
        self.engine.seed(seed as u32);
    }

    /// RandHelper::rand(SumoRNG*) -> [0,1)
    pub fn rand(&mut self) -> f64 {
        let res = self.engine.next_u32() as f64 / 4294967296.0;
        self.count += 1;
        res
    }
    pub fn rand_max(&mut self, max_v: f64) -> f64 {
        max_v * self.rand()
    }
    pub fn rand_range(&mut self, min_v: f64, max_v: f64) -> f64 {
        min_v + (max_v - min_v) * self.rand()
    }
    pub fn rand_int(&mut self, max_v: i32) -> i32 {
        let mut used = (max_v - 1) as u32;
        used |= used >> 1;
        used |= used >> 2;
        used |= used >> 4;
        used |= used >> 8;
        used |= used >> 16;
        loop {
            let result = (self.engine.next_u32() & used) as i32;
            self.count += 1;
            if result < max_v {
                return result;
            }
        }
    }
    pub fn rand_int_range(&mut self, min_v: i32, max_v: i32) -> i32 {
        min_v + self.rand_int(max_v - min_v)
    }
    pub fn rand_i64(&mut self, max_v: i64) -> i64 {
        if max_v <= i32::MAX as i64 {
            return self.rand_int(max_v as i32) as i64;
        }
        let mut used = (max_v - 1) as u64;
        for s in [1, 2, 4, 8, 16, 32] {
            used |= used >> s;
        }
        loop {
            let hi = self.engine.next_u32() as u64;
            let lo = self.engine.next_u32() as u64;
            let result = (((hi << 32) | lo) & used) as i64;
            self.count += 2;
            if result < max_v {
                return result;
            }
        }
    }
    pub fn rand_norm(&mut self, mean: f64, variance: f64) -> f64 {
        let (mut u, mut q);
        loop {
            u = self.rand_max(2.0) - 1.;
            let v = self.rand_max(2.0) - 1.;
            q = u * u + v * v;
            if !(q == 0.0 || q >= 1.0) {
                break;
            }
        }
        let log_rounded = (q.ln() * 1e14).ceil() / 1e14;
        mean + variance * u * (-2. * log_rounded / q).sqrt()
    }
    pub fn rand_exp(&mut self, rate: f64) -> f64 {
        let mut r = self.rand();
        while r == 0. {
            r = self.rand();
        }
        -r.ln() / rate
    }
    pub fn get_random_from<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        &v[self.rand_int(v.len() as i32) as usize]
    }
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        let mut i = v.len() as i32 - 1;
        while i > 0 {
            let j = self.rand_int(i) as usize;
            v.swap(i as usize, j);
            i -= 1;
        }
    }
    pub fn save_state(&self) -> String {
        // shortcut: full generator state dump (count >= 1e6) not supported yet; add when state saving is ported
        self.count.to_string()
    }
    pub fn load_state(&mut self, state: &str) {
        let c: u64 = state.split_whitespace().next().and_then(|s| s.parse().ok()).unwrap_or(0);
        self.count = c;
        self.engine.discard(c);
    }
}

thread_local! {
    static DEFAULT_RNG: RefCell<SumoRNG> = RefCell::new(SumoRNG::new("default"));
}

/// Access the global default RNG (RandHelper::myRandomNumberGenerator)
pub fn with_default<R>(f: impl FnOnce(&mut SumoRNG) -> R) -> R {
    DEFAULT_RNG.with(|r| f(&mut r.borrow_mut()))
}
pub fn rand() -> f64 {
    with_default(|r| r.rand())
}
pub fn init_rand(rng: Option<&mut SumoRNG>, random: bool, seed: i32) {
    let s = if random {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i32).unwrap_or(0)
    } else {
        seed
    };
    match rng {
        Some(r) => r.set_seed(s),
        None => with_default(|r| r.set_seed(s)),
    }
}

/// C++: `void RandHelper::insertRandOptions(OptionsCont& oc)`
pub fn insert_rand_options(oc: &mut OptionsCont) {
    oc.add_option_sub_topic("Random Number");

    oc.do_register("random", SumoOption::bool(false));
    oc.add_synonyme("random", "abs-rand", true);
    oc.add_description("random", "Random Number", "Initialises the random number generator with the current system time");

    oc.do_register("seed", SumoOption::integer(23423));
    oc.add_synonyme("seed", "srand", true);
    oc.add_description("seed", "Random Number", "Initialises the random number generator with the given value");
}

pub fn rand_hash(x: i64) -> f64 {
    let h = splitmix64(x as u64) >> 11;
    h as f64 / (1u64 << 53) as f64
}

fn murmur_32_scramble(mut k: u32) -> u32 {
    k = k.wrapping_mul(0xcc9e2d51);
    k = k.rotate_left(15);
    k.wrapping_mul(0x1b873593)
}

pub fn murmur3_32(key: &str, seed: i32) -> u32 {
    let key = key.as_bytes();
    let mut h = seed as u32;
    let mut chunks = key.chunks_exact(4);
    for c in &mut chunks {
        let k = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
        h ^= murmur_32_scramble(k);
        h = h.rotate_left(13);
        h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
    }
    let rest = chunks.remainder();
    let mut k = 0u32;
    for i in (0..rest.len()).rev() {
        k <<= 8;
        k |= rest[i] as u32;
    }
    h ^= murmur_32_scramble(k);
    h ^= key.len() as u32;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^ (h >> 16)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mt19937_matches_std() {
        // the C++ standard requires the 10000th output of a default-constructed mt19937 to be 4123659995
        let mut g = Mt19937::new(5489);
        g.discard(9999);
        assert_eq!(g.next_u32(), 4123659995);
    }
}
