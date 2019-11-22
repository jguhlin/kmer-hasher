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
        for _ in 0..10 {
            let mut kmer: Vec<u8> = Vec::new();
            for _ in 0..13 {
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
    let kmerx = KMERS_BV_U64[0].clone();
    assert_eq!(kmerx.as_slice().len(), 1);
    println!("{:0>64b}", u64::max_value());
    println!("{:0>64b}", kmerx.as_slice()[0] as u64);
    let kmery = !kmerx;
    println!("{:0>64b}", kmery.as_slice()[0] as u64);
}