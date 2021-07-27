// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(rustc_private)]
include!{"../rmc-prelude.rs"}

use std::env;

fn operate_on_rmc_vec(times: usize) {
    let mut v: RmcVec<u32> = RmcVec::new();
    for i in 0..times {
        v.push(__nondet());
    }
    assert!(v.len() == times);
    v.push(1);
    assert!(v.pop() == Some(1));
}

fn operate_on_c_vec(times: usize) {
    let mut v: CVec<u32> = CVec::new();
    for i in 0..times {
        v.push(__nondet());
    }
    assert!(v.len() == times);
    v.push(1);
    assert!(v.pop() == Some(1));
}

fn operate_on_std_vec(times: usize) {
    let mut v: Vec<u32> = Vec::new();
    for i in 0..times {
        v.push(__nondet());
    }
    assert!(v.len() == times);
    v.push(1);
    assert!(v.pop() == Some(1));
}

fn main() {
    // let t_key = "TIMES";
    // let v_key = "VECTYPE";
    // if let Ok(times) = env::var(t_key) {
    //     if let Ok(vtype) = env::var(v_key) {
    //         if vtype == "rmc_vec" {
    //             operate_on_rmc_vec(times.parse().unwrap());
    //         } else if vtype == "c_vec" {
    //             operate_on_c_vec(times.parse().unwrap());
    //         } else if vtype == "std_vec" {
    //             operate_on_std_vec(times.parse().unwrap());
    //         }
    //     } else {
    //         panic!("Vectype not found");
    //     }
    // } else {
    //     panic!("Could not find count");
    // }

    operate_on_c_vec(1000);
}
