# Benchmarks

```sh
SBPF_BENCH_PROGRAMS_DIR=<dir> cargo bench
```

## Baseline — `132411d`

`Executable::from_elf` (parse and, for v0, relocate) against
`Executable::verify`. Sizes in KiB, times are criterion mean estimates.

`relative_call_sbpfv0` (1.6 KiB, embedded): `load` 946 ns, `verify` 57 ns.
`relative_call` (0.8 KiB, embedded): `load` 72 ns, `verify` 48 ns. Too small to
be representative.

### v0

| Program | Size | `load` | `verify` | `verify`/`load` |
| --- | ---: | ---: | ---: | ---: |
| `D9ek6qwZgvbksJLzXeG9jaNFJgdp68A3iC5yLynieJQp` | 30 | 22.9 us | 24.8 us | 1.1x |
| `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr` | 73 | 60.8 us | 51.9 us | 0.9x |
| `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` | 102 | 76.7 us | 80.1 us | 1.0x |
| `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` | 106 | 59.5 us | 87.4 us | 1.5x |
| `D67re8wUwwZ12ni1fMbzaqwfcG3atiRrMMvptEZmENGs` | 199 | 141 us | 138 us | 1.0x |
| `LGDSXVcDx4Ynw7UXavGEe5nwzyUZZ5d3sLkwYk26LUf` | 450 | 350 us | 328 us | 0.9x |
| `darkr3FB87qAZmgLwKov6Hk9Yiah5UT4rUYu8Zhthw1` | 800 | 666 us | 552 us | 0.8x |
| `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | 1349 | 509 us | 506 us | 1.0x |
| `4MangoMjqJ2firMokCjjGgoK8d4MXcrgL7XJaL3w6fVg` | 3501 | 3.35 ms | 2.38 ms | 0.7x |
| `FLASH6Lo6h3iasJKWDs2F8TkW2UKf3s15C8PMGuVfgBn` | 6892 | 5.55 ms | 4.03 ms | 0.7x |
| `UMBRAD2ishebJTcgCLkTkNUx1v3GyoAgpTRPeWoLykh` | 7058 | 6.11 ms | 4.78 ms | 0.8x |

### v3

| Program | Size | `load` | `verify` | `verify`/`load` |
| --- | ---: | ---: | ---: | ---: |
| `FmGfWtigbVnYrqryq5z5GdCzCc3XcFa36h86P26qncr1` | 29 | 0.73 us | 18.2 us | 25x |
| `53o2tVBfNXj4DgmDKjUWPC9Hszw6zYNG11CY66irhU74` | 80 | 1.83 us | 69.8 us | 38x |
| `3XjiiaQhwpu1NccV4dVGc9LqbmKGqfJNCbSj3KnXyCSR` | 113 | 2.71 us | 84.4 us | 31x |
| `5zqNuvXY7yLtM1KsjxwFgFNxR56Kfzrg3auFdV7viEcP` | 198 | 5.08 us | 149 us | 29x |
| `vuHFdYXjv9ePz6CRGXyQRzRfRLX3yyT8zG5hGiUpwF6` | 341 | 10.4 us | 248 us | 24x |
| `45s36RbsPudmfu82YhE7WXDWzcyJvppfxKYUgCXM6sB5` | 676 | 21.7 us | 469 us | 22x |
| `FYaHz8zsZzZJetMmU1uxwfzkU8aryPoWyFsSbm69D44G` | 1166 | 34.7 us | 812 us | 23x |
| `LendVMybdnkGL9yX9VFJamrtCSzL3izpUoB9JDhSU6M` | 1185 | 46.2 us | 887 us | 19x |
| `CQwWoJENUtKmwCMqnyGbEYkg41oxdat23kkNdJLvY7v9` | 3345 | 137 us | 1.96 ms | 14x |

## `elf: skip full instruction decode in the call relocation pass`

v0 load: median **-47%**, best -69% (`Tokenkeg`), worst -38% (`4Mango`). v3 and
`verify` untouched.

`relocate` walked `.text` with `ebpf::get_insn`, which decodes all six fields of
every instruction and bounds-checks each read, to look at two of them. Reading
the opcode and the immediate out of fixed size chunks drops the decode and lets
the immediate be written back without re-checking the section bounds.

### v0

| Program | Size | `load` | Before | Δ |
| --- | ---: | ---: | ---: | ---: |
| `D9ek6qwZgvbksJLzXeG9jaNFJgdp68A3iC5yLynieJQp` | 30 | 12.1 us | 24.7 us | -51% |
| `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr` | 73 | 35.9 us | 60.2 us | -40% |
| `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` | 102 | 40.0 us | 71.3 us | -44% |
| `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` | 106 | 18.5 us | 59.4 us | -69% |
| `D67re8wUwwZ12ni1fMbzaqwfcG3atiRrMMvptEZmENGs` | 199 | 71.1 us | 141 us | -50% |
| `LGDSXVcDx4Ynw7UXavGEe5nwzyUZZ5d3sLkwYk26LUf` | 450 | 189 us | 379 us | -50% |
| `darkr3FB87qAZmgLwKov6Hk9Yiah5UT4rUYu8Zhthw1` | 800 | 349 us | 641 us | -46% |
| `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | 1349 | 289 us | 536 us | -46% |
| `4MangoMjqJ2firMokCjjGgoK8d4MXcrgL7XJaL3w6fVg` | 3501 | 2.07 ms | 3.34 ms | -38% |
| `FLASH6Lo6h3iasJKWDs2F8TkW2UKf3s15C8PMGuVfgBn` | 6892 | 2.89 ms | 4.99 ms | -42% |
| `UMBRAD2ishebJTcgCLkTkNUx1v3GyoAgpTRPeWoLykh` | 7058 | 3.15 ms | 6.44 ms | -51% |

v3 loads do not go through `relocate` and moved by -8%..+25%, which is the noise
floor for a path that is a `memcpy` and a handful of header checks.

## `program: index the function registry by hash instead of by order`

v0 load: median **-29%**, best -35% (`darkr3`), worst -18% (`D9ek6q`).
Cumulative against the baseline: **-61%** median. v3 within noise.

Every call site in `relocate` performs one registry operation, and each cost
42 ns against the `BTreeMap` -- four times the 10 ns of the symbol hash it looks
up. The keys are already symbol hashes or program counters, so a `HashMap` that
only spreads them over the hash space replaces the ordered tree. `iter` and
`keys` sort on the way out, since callers depend on a stable order and both are
cold paths.

### v0

| Program | Size | `load` | Before | Δ | vs baseline |
| --- | ---: | ---: | ---: | ---: | ---: |
| `D9ek6qwZgvbksJLzXeG9jaNFJgdp68A3iC5yLynieJQp` | 30 | 9.95 us | 12.1 us | -18% | -60% |
| `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr` | 73 | 25.4 us | 35.9 us | -29% | -58% |
| `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` | 102 | 30.2 us | 40.0 us | -25% | -58% |
| `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` | 106 | 14.9 us | 18.5 us | -19% | -75% |
| `D67re8wUwwZ12ni1fMbzaqwfcG3atiRrMMvptEZmENGs` | 199 | 55.2 us | 71.1 us | -22% | -61% |
| `LGDSXVcDx4Ynw7UXavGEe5nwzyUZZ5d3sLkwYk26LUf` | 450 | 125 us | 189 us | -34% | -67% |
| `darkr3FB87qAZmgLwKov6Hk9Yiah5UT4rUYu8Zhthw1` | 800 | 226 us | 349 us | -35% | -65% |
| `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | 1349 | 214 us | 289 us | -26% | -60% |
| `4MangoMjqJ2firMokCjjGgoK8d4MXcrgL7XJaL3w6fVg` | 3501 | 1.37 ms | 2.07 ms | -34% | -59% |
| `FLASH6Lo6h3iasJKWDs2F8TkW2UKf3s15C8PMGuVfgBn` | 6892 | 1.96 ms | 2.89 ms | -32% | -61% |
| `UMBRAD2ishebJTcgCLkTkNUx1v3GyoAgpTRPeWoLykh` | 7058 | 2.14 ms | 3.15 ms | -32% | -67% |

## `elf: register each call target once instead of once per call site`

v0 load: median **-25%**, best -42% (`4Mango`), worst -4% (`D9ek6q`).
Cumulative against the baseline: **-75%** median. v3 within noise.

Programs call the same function from many places -- `UMBRAD` has 40372 call
sites against 2517 distinct targets, and `4Mango` 29684 against 1338. The pass
hashed the target and went to the registry at every site, so caching the key per
target removes roughly fifteen out of every sixteen of those.

### v0

| Program | Size | `load` | Before | Δ | vs baseline |
| --- | ---: | ---: | ---: | ---: | ---: |
| `D9ek6qwZgvbksJLzXeG9jaNFJgdp68A3iC5yLynieJQp` | 30 | 9.50 us | 9.95 us | -4% | -61% |
| `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr` | 73 | 24.3 us | 25.4 us | -5% | -60% |
| `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` | 102 | 24.1 us | 30.2 us | -20% | -66% |
| `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` | 106 | 12.4 us | 14.9 us | -17% | -79% |
| `D67re8wUwwZ12ni1fMbzaqwfcG3atiRrMMvptEZmENGs` | 199 | 44.3 us | 55.2 us | -20% | -69% |
| `LGDSXVcDx4Ynw7UXavGEe5nwzyUZZ5d3sLkwYk26LUf` | 450 | 91.3 us | 125 us | -27% | -76% |
| `darkr3FB87qAZmgLwKov6Hk9Yiah5UT4rUYu8Zhthw1` | 800 | 163 us | 226 us | -28% | -75% |
| `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | 1349 | 160 us | 214 us | -25% | -70% |
| `4MangoMjqJ2firMokCjjGgoK8d4MXcrgL7XJaL3w6fVg` | 3501 | 793 us | 1.37 ms | -42% | -76% |
| `FLASH6Lo6h3iasJKWDs2F8TkW2UKf3s15C8PMGuVfgBn` | 6892 | 1.25 ms | 1.96 ms | -37% | -75% |
| `UMBRAD2ishebJTcgCLkTkNUx1v3GyoAgpTRPeWoLykh` | 7058 | 1.39 ms | 2.14 ms | -35% | -78% |

## `elf: let the caller hand over the ELF buffer`

v3 `load_owned`: median **-99%**, worst -87% (`FmGfWt`). v0 unaffected.

A strict load was measured at 93-103% `AlignedMemory::from_slice` -- it is a
`memcpy` and a handful of header checks, and nothing else. `load` has to copy
because the executable outlives the call, so `load_owned` takes an
`AlignedMemory` the caller already holds and keeps it. Only the strict parser
can use it: relocation reads the unrelocated ELF while writing the relocated
one, so the lenient parser needs a second buffer regardless.

This moves the copy rather than deleting it. A caller which only has a slice to
lend gains nothing and should keep using `load`. The gain is real only for one
which can fill an `AlignedMemory` directly, from a file or an account, and hand
it over. The figures below exclude building that buffer, which is the caller's
cost and happens in criterion's untimed setup.

### v3

| Program | Size | `load` | `load_owned` | Δ |
| --- | ---: | ---: | ---: | ---: |
| `FmGfWtigbVnYrqryq5z5GdCzCc3XcFa36h86P26qncr1` | 29 | 795 ns | 102 ns | -87% |
| `53o2tVBfNXj4DgmDKjUWPC9Hszw6zYNG11CY66irhU74` | 80 | 1.98 us | 99 ns | -95% |
| `3XjiiaQhwpu1NccV4dVGc9LqbmKGqfJNCbSj3KnXyCSR` | 113 | 2.60 us | 122 ns | -95% |
| `5zqNuvXY7yLtM1KsjxwFgFNxR56Kfzrg3auFdV7viEcP` | 198 | 5.09 us | 141 ns | -97% |
| `vuHFdYXjv9ePz6CRGXyQRzRfRLX3yyT8zG5hGiUpwF6` | 341 | 11.6 us | 165 ns | -99% |
| `45s36RbsPudmfu82YhE7WXDWzcyJvppfxKYUgCXM6sB5` | 676 | 22.2 us | 193 ns | -99% |
| `FYaHz8zsZzZJetMmU1uxwfzkU8aryPoWyFsSbm69D44G` | 1166 | 46.8 us | 192 ns | -100% |
| `LendVMybdnkGL9yX9VFJamrtCSzL3izpUoB9JDhSU6M` | 1185 | 41.3 us | 184 ns | -100% |
| `CQwWoJENUtKmwCMqnyGbEYkg41oxdat23kkNdJLvY7v9` | 3345 | 138 us | 342 ns | -100% |
