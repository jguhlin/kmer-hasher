extern crate rand;
extern crate seahash;
extern crate fnv;
extern crate twox_hash;
extern crate t1ha;
extern crate bitvec;

use twox_hash::XxHash64;

use rand::Rng;
use wyhash::wyhash;
use seahash::SeaHasher;
use std::time::Duration;
use std::hash::Hasher;
use fnv::FnvHasher;
use t1ha::{t1ha0, t1ha2_atonce, T1ha2Hasher};
use bitvec::prelude::*;

use std::ops::BitXor;

#[macro_use]
extern crate lazy_static;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

lazy_static! {

    static ref KMERS: Vec<Vec<u8>> = {

        let mut rng = rand::thread_rng();

        let mut kmers: Vec<Vec<u8>> = Vec::new();
        for _ in 0..100_000 {
            let mut kmer: Vec<u8> = Vec::new();
            for _ in 0..21 {
                let x = rng.gen_range(0, 5);
                kmer.push(
                    match x {
                        0 => b'A',
                        1 => b'C',
                        2 => b'T',
                        3 => b'G',
                        4 => b'N',
                        _ => unreachable!(),
                    });
            }
            kmers.push(kmer);
        }
        kmers
    };

    static ref KMERS_BV_U64: Vec<BitVec<LittleEndian, u64>> = {
        let mut kmers_bv_u64: Vec<BitVec<LittleEndian, u64>> = Vec::with_capacity(KMERS.len());

        for kmer in KMERS.clone().iter() {
            let mut bv: BitVec<LittleEndian, u64> = BitVec::<LittleEndian, u64>::new();
            for x in kmer.iter() {
                match x {
                    b'A' => bv.extend(bitvec![0, 0, 0]),
                    b'T' => bv.extend(bitvec![0, 1, 1]),
                    b'C' => bv.extend(bitvec![0, 0, 1]),
                    b'G' => bv.extend(bitvec![0, 1, 0]),
                    b'N' => bv.extend(bitvec![1, 1, 1]),
                    _    => unreachable!(),
                }
            }
            kmers_bv_u64.push(bv);
        }
        kmers_bv_u64
    };

    static ref KMERS_BV: Vec<BitVec<LittleEndian, u8>> = {
        let mut kmers_bv: Vec<BitVec<LittleEndian, u8>> = Vec::with_capacity(KMERS.len());

        for kmer in KMERS.clone().iter() {
            let mut bv: BitVec<LittleEndian, u8> = BitVec::<LittleEndian, u8>::new();
            for x in kmer.iter() {
                match x {
                    b'A' => bv.extend(bitvec![0, 0, 0]),
                    b'T' => bv.extend(bitvec![0, 1, 1]),
                    b'C' => bv.extend(bitvec![0, 0, 1]),
                    b'G' => bv.extend(bitvec![0, 1, 0]),
                    b'N' => bv.extend(bitvec![1, 1, 1]),
                    _    => unreachable!(),
                }
            }
            kmers_bv.push(bv);
        }
        kmers_bv
    };
}

fn bench_wyhash(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS.len());
    c.bench_function("wyhash hashing", |b| b.iter(|| {
        for kmer in KMERS.clone().iter() {
            hashes.push(wyhash(&kmer, 43_988_123));
        }
    }));
}

fn bench_wyhash_bv(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS_BV.len());
    c.bench_function("bv: wyhash hashing", |b| b.iter(|| {
        for kmer in KMERS_BV.clone().iter() {
            hashes.push(wyhash(kmer.as_slice(), 43_988_123));
        }
    }));
}

fn bench_seahash(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS.len());
    c.bench_function("seahash hashing", |b| b.iter(|| {
        for kmer in KMERS.clone().iter() {
            hashes.push(seahash::hash_seeded(&kmer, 42_988_123, 1_328_433, 193_235_245, 184_124));
        }
    }));
}

fn bench_seahash_bv(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS_BV.len());
    c.bench_function("bv: seahash hashing", |b| b.iter(|| {
        for kmer in KMERS_BV.clone().iter() {
            hashes.push(seahash::hash_seeded(kmer.as_slice(), 42_988_123, 1_328_433, 193_235_245, 184_124));
        }
    }));
}

fn bench_xxhash(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS.len());
    c.bench_function("xxhash hashing", |b| b.iter(|| {
        for kmer in KMERS.clone().iter() {
            let mut hasher = XxHash64::with_seed(0xae05_4331_1b70_2d91);
            hasher.write(&kmer);
            hashes.push(hasher.finish());
        }
    }));
}

fn bench_xxhash_bv(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS_BV.len());
    c.bench_function("bv: xxhash hashing", |b| b.iter(|| {
        for kmer in KMERS_BV.clone().iter() {
            let mut hasher = XxHash64::with_seed(0xae05_4331_1b70_2d91);
            hasher.write(kmer.as_slice());
            hashes.push(hasher.finish());
        }
    }));
}

fn bench_fnvhash(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS.len());
    c.bench_function("fnv hashing", |b| b.iter(|| {
        for kmer in KMERS.clone().iter() {
            let mut hasher = FnvHasher::with_key(42_988_123);
            hasher.write(&kmer);
            hashes.push(hasher.finish());
        }
    }));
}

fn bench_fnvhash_bv(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS_BV.len());
    c.bench_function("bv: fnv hashing", |b| b.iter(|| {
        for kmer in KMERS_BV.clone().iter() {
            let mut hasher = FnvHasher::with_key(42_988_123);
            hasher.write(kmer.as_slice());
            hashes.push(hasher.finish());
        }
    }));
}

fn bench_t1ha0(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS.len());
    c.bench_function("t1ha0 hashing", |b| b.iter(|| {
        for kmer in KMERS.clone().iter() {
            hashes.push(t1ha0(&kmer, 42_988_123));
        }
    }));
}

fn bench_t1ha0_bv(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS_BV.len());
    c.bench_function("bv: t1ha0 hashing", |b| b.iter(|| {
        for kmer in KMERS_BV.clone().iter() {
            hashes.push(t1ha0(kmer.as_slice(), 42_988_123));
        }
    }));
}

fn bench_3bit(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS.len());
    c.bench_function("3bit hashing", |b| b.iter(|| {
        for kmer in KMERS.clone().iter() {
            hashes.push(convert_kmer_to_bits(21, &kmer));
        }
    }));
}

fn hash_vec_u8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hashing Vec<u8>");

    group.bench_function("t1ha0", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            hashes.push(t1ha0(&kmer, 42_988_123));
        }}));

    group.bench_function("fnv", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            let mut hasher = FnvHasher::with_key(42_988_123);
            hasher.write(&kmer);
            hashes.push(hasher.finish());
        }}));

    group.bench_function("xxhash", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            let mut hasher = XxHash64::with_seed(0xae05_4331_1b70_2d91);
            hasher.write(&kmer);
            hashes.push(hasher.finish());
        }}));

    group.bench_function("seahash", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            hashes.push(seahash::hash_seeded(&kmer, 42_988_123, 1_328_433, 193_235_245, 184_124));
        }}));

    group.bench_function("wyhash", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            hashes.push(wyhash(&kmer, 43_988_123));
        }}));

    group.finish();
}

fn hash_vec_bv(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hashing Vec<Bitvec>");

    group.bench_function("t1ha0", |b| b.iter(|| {
        let kmers = KMERS_BV.clone();
        let mut hashes = Vec::with_capacity(KMERS_BV.len());
        for kmer in kmers {
            hashes.push(t1ha0(kmer.as_slice(), 42_988_123));
        }}));

    group.bench_function("fnv", |b| b.iter(|| {
        let kmers = KMERS_BV.clone();
        let mut hashes = Vec::with_capacity(KMERS_BV.len());
        for kmer in kmers {
            let mut hasher = FnvHasher::with_key(42_988_123);
            hasher.write(kmer.as_slice());
            hashes.push(hasher.finish());
        }}));

    group.bench_function("xxhash", |b| b.iter(|| {
        let kmers = KMERS_BV.clone();
        let mut hashes = Vec::with_capacity(KMERS_BV.len());
        for kmer in kmers {
            let mut hasher = XxHash64::with_seed(0xae05_4331_1b70_2d91);
            hasher.write(kmer.as_slice());
            hashes.push(hasher.finish());
        }}));

    group.bench_function("seahash", |b| b.iter(|| {
        let kmers = KMERS_BV.clone();
        let mut hashes = Vec::with_capacity(KMERS_BV.len());
        for kmer in kmers {
            hashes.push(seahash::hash_seeded(kmer.as_slice(), 42_988_123, 1_328_433, 193_235_245, 184_124));
        }}));

    group.bench_function("wyhash", |b| b.iter(|| {
        let kmers = KMERS_BV.clone();
        let mut hashes = Vec::with_capacity(KMERS_BV.len());
        for kmer in kmers {
            hashes.push(wyhash(kmer.as_slice(), 43_988_123));
        }}));

    group.bench_function("custom add", |b| b.iter(|| {
        let kmers = KMERS_BV_U64.clone();
        let mut hashes = Vec::with_capacity(KMERS_BV_U64.len());
        for kmer in kmers {
            let val: u64 = 0;
            hashes.push(kmer.as_slice().iter().fold(0_u64, |acc, x| acc.wrapping_add(*x as u64)));
        }}));

    group.bench_function("custom xor", |b| b.iter(|| {
        let kmers = KMERS_BV_U64.clone();
        let mut hashes = Vec::with_capacity(KMERS_BV_U64.len());
        for kmer in kmers {
            let val: u64 = 0;
            hashes.push(kmer.as_slice().iter().fold(0_u64, |acc, x| acc.bitxor(x)));
        }}));

    group.bench_function("custom take u64", |b| b.iter(|| {
        let kmers = KMERS_BV_U64.clone();
        let mut hashes = Vec::with_capacity(KMERS_BV_U64.len());
        for kmer in kmers {
            hashes.push(kmer.as_slice()[0]);
        }}));

    group.finish();
}


fn custom_criterion() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(75))
        .sample_size(100)
}

/* criterion_group! { 
    name = hashing_benches;
    config = custom_criterion();
    targets = bench_wyhash, bench_seahash, bench_xxhash, bench_fnvhash, bench_t1ha0
} */

criterion_group! { 
    name = hashing_benches;
    config = custom_criterion();
    targets = hash_vec_u8
}

criterion_group! { 
    name = hashing_benches_bv;
    config = custom_criterion();
    targets = hash_vec_bv
} 

/* criterion_group! { 
    name = hashing_benches_bv;
    config = custom_criterion();
    targets = bench_wyhash_bv, bench_seahash_bv, bench_xxhash_bv, bench_fnvhash_bv, bench_t1ha0_bv
} */

criterion_main!(hashing_benches, hashing_benches_bv);
// criterion_main!(hashing_benches_bv);

fn convert_kmer_to_bits(k: usize, kmer: &[u8]) -> u64 {
    let mut bits: u64 = 0;
    
    assert!(k <= 21);
    
    let iter = kmer.iter().rev().enumerate();
    
    for (n, base) in iter {
        change_bits(*base, n, &mut bits);
    }

    bits
}

/* A => 111
 * T => 000
 * C => 101
 * G => 010
 * N => 001
 * N => 100 // Need RC of N, which is N
*/

fn change_bits(char: u8, n: usize, bits: &mut u64) -> u64 {
    match char {
        b'A' => *bits |= 0b111 << n * 3,
        b'T' => *bits |= 0b000 << n * 3,
        b'C' => *bits |= 0b101 << n * 3,
        b'G' => *bits |= 0b010 << n * 3,
        b'N' => *bits |= 0b001 << n * 3,
        _    => *bits |= 0b001 << n * 3,
    }
    *bits
}