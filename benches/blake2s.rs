/*******************************************************************************
  Copyright 2019 Supranational LLC

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*******************************************************************************/
#![feature(test)]

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

extern crate test;

#[bench]
fn blake2s_filecoin_benchmark(b: &mut ::test::Bencher) {
    let mut rng: SmallRng = SeedableRng::seed_from_u64(1);
 
    let rep: [u8; 32] = rng.gen();
    let  p0: [u8; 32] = rng.gen();
    let  p1: [u8; 32] = rng.gen();
    let  p2: [u8; 32] = rng.gen();
    let  p3: [u8; 32] = rng.gen();
    let  p4: [u8; 32] = rng.gen();
    let  p5: [u8; 32] = rng.gen();
    let  p6: [u8; 32] = rng.gen();
    let  p7: [u8; 32] = rng.gen();
    let  p8: [u8; 32] = rng.gen();
    let  p9: [u8; 32] = rng.gen();
    let p10: [u8; 32] = rng.gen();
    let p11: [u8; 32] = rng.gen();
    let p12: [u8; 32] = rng.gen();

    let parents: [&[u8]; 14] = [&rep, &p0, &p1, &p2,  &p3,  &p4,  &p5,
                                 &p6, &p7, &p8, &p9, &p10, &p11, &p12];

    b.bytes = 448;
    b.iter(|| blake2s_filecoin::hash_nodes_14(&parents));
}
