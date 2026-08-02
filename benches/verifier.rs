// Copyright 2020 Solana Maintainers <maintainers@solana.com>
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license <http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use {
    criterion::{criterion_group, criterion_main, Criterion, Throughput},
    solana_sbpf::{elf::Executable, program::BuiltinProgram, verifier::RequisiteVerifier},
    std::sync::Arc,
    test_utils::{program::programs, TestContextObject},
};

fn bench_verify(c: &mut Criterion) {
    let loader = Arc::new(BuiltinProgram::new_mock());

    for program in programs() {
        // Loading is the precondition for verifying, so it happens outside of
        // the timed section.
        let executable =
            Executable::<TestContextObject>::from_elf(&program.elf, loader.clone()).unwrap();

        let mut group = c.benchmark_group(&program.name);
        // Only the text section is verified, so the rest of the ELF would skew
        // the throughput.
        group.throughput(Throughput::Bytes(executable.get_text_bytes().1.len() as u64));
        group.bench_function("verify", |b| {
            b.iter(|| executable.verify::<RequisiteVerifier>().unwrap())
        });
        group.finish();
    }
}

criterion_group!(benches, bench_verify);
criterion_main!(benches);
