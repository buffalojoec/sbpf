// Copyright 2020 Solana Maintainers <maintainers@solana.com>
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license <http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use {
    criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput},
    solana_sbpf::{
        aligned_memory::AlignedMemory, ebpf::HOST_ALIGN, elf::Executable, program::BuiltinProgram,
    },
    std::sync::Arc,
    test_utils::{program::programs, TestContextObject},
};

fn bench_elf_load(c: &mut Criterion) {
    let loader = Arc::new(BuiltinProgram::new_mock());

    for program in programs() {
        // The two loading paths are told apart by the `v0`/`v3` prefix the
        // corpus gives each program.
        let mut group = c.benchmark_group(&program.name);
        group.throughput(Throughput::Bytes(program.elf.len() as u64));
        group.bench_function("load", |b| {
            // Executables are dropped inside the timed section. Holding them
            // instead retains gigabytes over a run, and the cost becomes the
            // kernel faulting in fresh pages rather than the loader itself.
            b.iter(|| {
                Executable::<TestContextObject>::from_elf(&program.elf, loader.clone()).unwrap()
            })
        });
        // Only the strict parser can use a handed over buffer, so benchmarking
        // the lenient ones here would just report the batching overhead of
        // allocating a fresh buffer per iteration.
        if !program.is_strict() {
            group.finish();
            continue;
        }
        group.bench_function("load_owned", |b| {
            // Building the buffer happens in the setup, which criterion does
            // not time. This is therefore the cost to a caller which already
            // holds the ELF in an `AlignedMemory` and hands it over, not to one
            // which only has a slice to lend.
            b.iter_batched(
                || AlignedMemory::<{ HOST_ALIGN }>::from_slice(&program.elf),
                |elf| Executable::<TestContextObject>::load_owned(elf, loader.clone()).unwrap(),
                BatchSize::PerIteration,
            )
        });
        group.finish();
    }
}

criterion_group!(benches, bench_elf_load);
criterion_main!(benches);
