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
