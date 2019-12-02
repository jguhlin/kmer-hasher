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

use std::mem;

#[macro_use]
extern crate lazy_static;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

lazy_static! {

    static ref CONVERSION: [u64; 256] = {

        let mut conversion: [u64; 256] = [1; 256];
        conversion[65]  = 7;
        conversion[97]  = 7;
        conversion[84]  = 0;
        conversion[116] = 0;
        conversion[67]  = 5;
        conversion[99]  = 5;
        conversion[71]  = 2;
        conversion[103] = 2;

        conversion
    };

    static ref CONVERSION_I: [i64; 256] = {

        let mut conversion: [i64; 256] = [1; 256];
        conversion[65]  = 7;
        conversion[97]  = 7;
        conversion[84]  = 0;
        conversion[116] = 0;
        conversion[67]  = 5;
        conversion[99]  = 5;
        conversion[71]  = 2;
        conversion[103] = 2;

        conversion
    };


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

/* fn bench_3bit(c: &mut Criterion) {
    let mut hashes = Vec::with_capacity(KMERS.len());
    c.bench_function("3bit hashing", |b| b.iter(|| {
        for kmer in KMERS.clone().iter() {
            hashes.push(convert_kmer_to_bits(21, &kmer));
        }
    }));
} */

fn hash_vec_u8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hashing Vec<u8>");

/*     group.bench_function("3bit3", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            hashes.push(convert_kmer_to_bits3(&kmer));
        }})); */

    // So much slower!
    /* group.bench_function("3bit2_hash4_conversionfn", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        let chunks = kmers.chunks_exact(4);

        for kmer in chunks.remainder() {
            hashes.push(convert_kmer_to_bits2_foreach(&kmer));
        }

        for chunk in chunks {
            let i: (u64, u64, u64, u64) = hash4_conversionfn(&chunk[0], &chunk[1], &chunk[2], &chunk[3]);
            hashes.push(i.0);
            hashes.push(i.1);
            hashes.push(i.2);
            hashes.push(i.3);
        }}));     */

    group.bench_function("3bit2_hash4", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        let chunks = kmers.chunks_exact(4);

        for kmer in chunks.remainder() {
            hashes.push(convert_kmer_to_bits2_foreach(&kmer));
        }

        for chunk in chunks {
            let i: (u64, u64, u64, u64) = hash4(&chunk[0], &chunk[1], &chunk[2], &chunk[3]);
            hashes.push(i.0);
            hashes.push(i.1);
            hashes.push(i.2);
            hashes.push(i.3);
        }}));

    group.bench_function("3bit2fe", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            hashes.push(convert_kmer_to_bits2_foreach(&kmer));
        }}));

    group.bench_function("3bit2", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            hashes.push(convert_kmer_to_bits2(&kmer));
        }}));
    
    group.bench_function("3bit2_1", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            hashes.push(convert_kmer_to_bits2_1(&kmer));
        }}));

    group.bench_function("3bit2_2", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            hashes.push(convert_kmer_to_bits2_2(&kmer));
    }}));

    // Even if slower could be faster due to faster RC
    // 23.1ms vs 6.3ms in 3bit2
    /* group.bench_function("3bit", |b| b.iter(|| {
        let kmers = KMERS.clone();
        let mut hashes = Vec::with_capacity(KMERS.len());
        for kmer in kmers {
            hashes.push(convert_kmer_to_bits(21, &kmer));
        }})); */

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
        .measurement_time(Duration::from_secs(60))
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

// criterion_main!(hashing_benches, hashing_benches_bv);
criterion_main!(hashing_benches);

// RETIRED
/*
fn convert_kmer_to_bits(k: usize, kmer: &[u8]) -> u64 {
    let mut bits: u64 = 0;
 
    let iter = kmer.iter().rev().enumerate();
    
    for (n, base) in iter {
        change_bits(*base, n, &mut bits);
    }

    bits
} */

#[inline(always)]
fn convert_kmer_to_bits2(kmer: &[u8]) -> u64 {
    let mut bits: u64 = 0;
    bits = bits.wrapping_add(CONVERSION[usize::from(kmer[0])]);
    for base in &kmer[1..] {
        bits <<= 3;
        bits = bits.wrapping_add(CONVERSION[usize::from(*base)]);
    }
    bits
}

#[inline(always)]
fn convert_kmer_to_bits2_foreach(kmer: &[u8]) -> u64 {
    let mut bits: u64 = 0;
    bits = bits.wrapping_add(CONVERSION[usize::from(kmer[0])]);
    &kmer[1..].iter().for_each(|base| {
        bits <<= 3;
        bits = bits.wrapping_add(CONVERSION[usize::from(*base)]);
    });
    bits
}


// Previous version, replaced because .wrapping_add is faster
// Here for legacy reasons...
fn convert_kmer_to_bits2_1(kmer: &[u8]) -> u64 {
    let mut bits: u64 = 0;
    bits += CONVERSION[usize::from(kmer[0])];
    for base in &kmer[1..] {
        bits <<= 3;
        bits += CONVERSION[usize::from(*base)];
    }
    bits
}

fn convert_kmer_to_bits2_2(kmer: &[u8]) -> u64 {
    let mut bits: u64 = 0;
    bits = bits.saturating_add(CONVERSION[usize::from(kmer[0])]);
    for base in &kmer[1..] {
        bits <<= 3;
        bits = bits.saturating_add(CONVERSION[usize::from(*base)]);
    }
    bits
}

// Not faster at this time...
fn convert_kmer_to_bits3(kmer: &[u8]) -> u64 {
    let mut to_add: [u64; 21] = [0; 21];

    for (n, base) in kmer.iter().enumerate() {
        to_add[n] = CONVERSION[*base as usize] << n * 3;
    }

    to_add.iter().sum()
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

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;

// AVX can calc 4 at a time
fn hash4(k1: &[u8], k2: &[u8], k3: &[u8], k4: &[u8]) -> (u64, u64, u64, u64) {
    unsafe {
        let mut hashes = _mm256_setzero_si256();
        let shift = _mm_set1_epi64x(3);

        let mut add = _mm256_set_epi64x(CONVERSION_I[usize::from(k1[0])],
                                        CONVERSION_I[usize::from(k2[0])],
                                        CONVERSION_I[usize::from(k3[0])],
                                        CONVERSION_I[usize::from(k4[0])]);
        
        hashes = _mm256_add_epi64(hashes, add);

        for i in 1..k1.len() {
            hashes = _mm256_sll_epi64(hashes, shift);
            add = _mm256_set_epi64x(CONVERSION_I[usize::from(k1[i])],
                                    CONVERSION_I[usize::from(k2[i])],
                                    CONVERSION_I[usize::from(k3[i])],
                                    CONVERSION_I[usize::from(k4[i])]);
            hashes = _mm256_add_epi64(hashes, add);
        }

        mem::transmute(hashes)
    }
}

// A is 7
// T is 0
// C is 5
// G is 2
// N is thus: 1
// N is also: 4... I mean 3

/*
fn conversion_i(n: &u8) -> i64 {
    match *n {
        b'A' => 7,
        b'a' => 7,
        b'T' => 0,
        b't' => 0,
        b'C' => 5,
        b'c' => 5,
        b'G' => 2,
        b'g' => 2,
        _    => 1
    }
}

fn hash4_conversionfn(k1: &[u8], k2: &[u8], k3: &[u8], k4: &[u8]) -> (u64, u64, u64, u64) {
    unsafe {
        let mut hashes = _mm256_setzero_si256();
        let shift = _mm_set1_epi64x(3);

        let mut add = _mm256_set_epi64x(conversion_i(&k1[0]),
                                        conversion_i(&k2[0]),
                                        conversion_i(&k3[0]),
                                        conversion_i(&k4[0]),);
        
        hashes = _mm256_add_epi64(hashes, add);

        for i in 1..k1.len() {
            hashes = _mm256_sll_epi64(hashes, shift);
            add = _mm256_set_epi64x(conversion_i(&k1[i]),
                                    conversion_i(&k2[i]),
                                    conversion_i(&k3[i]),
                                    conversion_i(&k4[i]));
            hashes = _mm256_add_epi64(hashes, add);
        }

        mem::transmute(hashes)
    }
}*/