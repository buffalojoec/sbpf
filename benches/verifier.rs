// Copyright 2020 Solana Maintainers <maintainers@solana.com>
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license <http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![feature(test)]

extern crate solana_sbpf;
extern crate test;
extern crate test_utils;

use solana_sbpf::{elf::Executable, program::BuiltinProgram, verifier::RequisiteVerifier};
use std::{fs, sync::Arc};
use test::Bencher;
use test_utils::TestContextObject;

/// The same program in both encodings, so that a change to code shared by the
/// two SBPF versions can be evaluated against both at once.
#[bench]
fn bench_verify_sbpfv0(bencher: &mut Bencher) {
    let elf = fs::read("tests/elfs/relative_call_sbpfv0.so").unwrap();
    let executable =
        Executable::<TestContextObject>::from_elf(&elf, Arc::new(BuiltinProgram::new_mock()))
            .unwrap();
    bencher.iter(|| executable.verify::<RequisiteVerifier>().unwrap());
}

#[bench]
fn bench_verify_sbpfv3(bencher: &mut Bencher) {
    let elf = fs::read("tests/elfs/relative_call.so").unwrap();
    let executable =
        Executable::<TestContextObject>::from_elf(&elf, Arc::new(BuiltinProgram::new_mock()))
            .unwrap();
    bencher.iter(|| executable.verify::<RequisiteVerifier>().unwrap());
}
