extern crate rand;
extern crate seahash;
extern crate fnv;
extern crate twox_hash;
extern crate t1ha;
extern crate bitvec;

use std::{i64, u64, mem};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;

use rand::Rng;
use bitvec::prelude::*;

#[macro_use]
extern crate lazy_static;


/* A => 111
 * T => 000
 * C => 101
 * G => 010
 * N => 001
 * N => 100 // Need RC of N, which is N
*/

// A is 7
// T is 0
// C is 5
// G is 2
// N is thus: 1
// N is also: 4

lazy_static! {

    static ref CONVERSION: [i64; 256] = {

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
        for _ in 0..10 {
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

fn main() {
    let hashes = hash4((&KMERS[0], &KMERS[1], &KMERS[2], &KMERS[3]));
    println!("{:0>64b}\n{:0>64b}\n{:0>64b}\n{:0>64b}", hashes.0, hashes.1, hashes.2, hashes.3);

}

// use std::cmp::min;

// A is 7
// T is 0
// C is 5
// G is 2
// N is thus: 1
// N is also: 4... I mean 3

/*
#[inline(always)]
pub fn kmerhash(kmer: &[u8]) -> u64 {
    let mut bits: u64 = 0;
    bits = bits.wrapping_add(CONVERSION[usize::from(kmer[0])]);
    &kmer[1..].iter().for_each(|base| {
        bits <<= 3;
        bits = bits.wrapping_add(CONVERSION[usize::from(*base)]);
    });
    bits
}

#[inline(always)]
pub fn kmerhash_smallest(kmer: &[u8]) -> u64 {
    let hash = kmerhash(kmer);
    let rc = calc_rc(kmer.len(), hash);
    min(hash, rc)
}

#[inline(always)]
pub fn calc_rc(k: usize, khash: u64) -> u64 {
    // khash is a kmer already processed with kmerhash
    // k is the k in kmer (thus the seq length)
    let mut rc = !khash.reverse_bits();
    rc >> (64 - (k * 3))
} */

// AVX can calc 4 at a time
fn hash4(kmers: (&[u8], &[u8], &[u8], &[u8])) -> (u64, u64, u64, u64) {
    unsafe {
        let mut hashes = _mm256_setzero_si256();
        let shift = _mm_set1_epi64x(3);

        let mut add = _mm256_set_epi64x(CONVERSION[usize::from(kmers.0[0])],
                                        CONVERSION[usize::from(kmers.1[0])],
                                        CONVERSION[usize::from(kmers.2[0])],
                                        CONVERSION[usize::from(kmers.3[0])]);
        
        hashes = _mm256_add_epi64(hashes, add);

        for i in 1..kmers.0.len() {
            hashes = _mm256_sll_epi64(hashes, shift);
            add = _mm256_set_epi64x(CONVERSION[usize::from(kmers.0[i])],
                                    CONVERSION[usize::from(kmers.1[i])],
                                    CONVERSION[usize::from(kmers.2[i])],
                                    CONVERSION[usize::from(kmers.3[i])]);
            hashes = _mm256_add_epi64(hashes, add);
        }

        mem::transmute(hashes)
    }
}