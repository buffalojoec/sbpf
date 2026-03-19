//! Assembly corpus for verifier violation testing.
//!
//! Each constant contains a minimal assembly program that triggers a specific
//! [`VerifierError`] variant. These programs are intended to be assembled into
//! ELF bytes and fed through the deploy / verification pipeline to observe
//! error mapping behaviour.
//!
//! Every program is self-contained and targets a single verifier check.
//! Programs for SBPFv0/v1 use `add64 r10, 0` as a no-op first instruction
//! because the assembler emits it as the manual stack-frame bump required
//! by those versions.

// ---------------------------------------------------------------------------
// VerifierError::DivisionByZero
// ---------------------------------------------------------------------------
/// Division by zero (imm). Triggers `DivisionByZero(0)`.
pub const DIVISION_BY_ZERO: &str = "\
    div32 r0, 0
    exit";

// ---------------------------------------------------------------------------
// VerifierError::UnsupportedLEBEArgument
// ---------------------------------------------------------------------------
// Cannot express via assembler because the assembler validates the immediate.
// We provide raw bytes instead (see `raw_corpus`).

// ---------------------------------------------------------------------------
// VerifierError::LDDWCannotBeLast
// ---------------------------------------------------------------------------
// Only one instruction (lddw) with no room for the second slot.
// Must be assembled from raw bytes for V0.

// ---------------------------------------------------------------------------
// VerifierError::IncompleteLDDW
// ---------------------------------------------------------------------------
// lddw followed by a non-zero opcode slot — raw bytes only.

// ---------------------------------------------------------------------------
// VerifierError::JumpOutOfCode
// ---------------------------------------------------------------------------
/// Forward jump past end of program. Triggers `JumpOutOfCode(_, 0)`.
pub const JUMP_OUT_OF_CODE: &str = "\
    ja +100
    exit";

// ---------------------------------------------------------------------------
// VerifierError::JumpToMiddleOfLDDW
// ---------------------------------------------------------------------------
/// Jump landing on the second slot of an lddw. Triggers `JumpToMiddleOfLDDW`.
/// Requires SBPFVersion::V0 (lddw is available).
pub const JUMP_TO_MIDDLE_OF_LDDW: &str = "\
    ja +1
    lddw r0, 0x1122334455667788
    exit";

// ---------------------------------------------------------------------------
// VerifierError::InvalidSourceRegister
// ---------------------------------------------------------------------------
// Register > 10 in source — can only be expressed with raw bytes.

// ---------------------------------------------------------------------------
// VerifierError::CannotWriteR10
// ---------------------------------------------------------------------------
/// Write to r10 (not via add64 in V1). Triggers `CannotWriteR10(0)`.
pub const CANNOT_WRITE_R10: &str = "\
    mov r10, 1
    exit";

// ---------------------------------------------------------------------------
// VerifierError::InvalidDestinationRegister
// ---------------------------------------------------------------------------
/// Destination register > 10. Triggers `InvalidDestinationRegister(0)`.
pub const INVALID_DESTINATION_REGISTER: &str = "\
    mov r11, 1
    exit";

// ---------------------------------------------------------------------------
// VerifierError::UnknownOpCode
// ---------------------------------------------------------------------------
// An unknown opcode — raw bytes only since the assembler won't emit one.

// ---------------------------------------------------------------------------
// VerifierError::ShiftWithOverflow
// ---------------------------------------------------------------------------
/// 32-bit shift by 32. Triggers `ShiftWithOverflow(32, 32, 0)`.
pub const SHIFT_WITH_OVERFLOW_32: &str = "\
    lsh32 r0, 32
    exit";

/// 64-bit shift by 64. Triggers `ShiftWithOverflow(64, 64, 0)`.
pub const SHIFT_WITH_OVERFLOW_64: &str = "\
    lsh64 r0, 64
    exit";

// ---------------------------------------------------------------------------
// VerifierError::InvalidRegister (callx with r10)
// ---------------------------------------------------------------------------
/// callx r10 is invalid. Triggers `InvalidRegister(0)`.
pub const INVALID_REGISTER_CALLX: &str = "\
    callx r10
    exit";

// ---------------------------------------------------------------------------
// VerifierError::UnalignedImmediate
// ---------------------------------------------------------------------------
/// Unaligned stack bump (r10 + odd value) in V1.
/// Triggers `UnalignedImmediate(0)`.
/// Requires SBPFVersion::V1 (manual_stack_frame_bump + alignment check).
pub const UNALIGNED_IMMEDIATE: &str = "\
    add r10, -63
    exit";

// ---------------------------------------------------------------------------
// Raw byte corpus — for violations the assembler refuses to emit.
// ---------------------------------------------------------------------------
/// Helpers for violations that require hand-crafted bytecode.
pub mod raw {
    use solana_sbpf::ebpf;

    /// UnsupportedLEBEArgument: BE instruction with immediate 3 (not 16/32/64).
    /// Two instructions: `be r1, 3` + `exit`.
    pub const UNSUPPORTED_LEBE_ARG: &[u8] = &[
        // be r1, 3  (opcode=0xdc, dst=1, src=0, off=0, imm=3)
        0xdc, 0x01, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, // exit
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    /// LDDWCannotBeLast: A single lddw instruction with no room for second slot.
    /// Only 8 bytes total.
    pub const LDDW_CANNOT_BE_LAST: &[u8] = &[
        // lddw r0, 0x55667788 (first 8 bytes only)
        0x18, 0x00, 0x00, 0x00, 0x88, 0x77, 0x66, 0x55,
    ];

    /// IncompleteLDDW: lddw followed by a non-zero opcode in the second slot.
    pub const INCOMPLETE_LDDW: &[u8] = &[
        // lddw r0, 0x55667788 (first slot)
        0x18, 0x00, 0x00, 0x00, 0x88, 0x77, 0x66, 0x55,
        // second slot has non-zero opcode (0x85 = call)
        0x85, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    /// InvalidSourceRegister: mov r0, r12 (src = 12 > 10).
    pub const INVALID_SOURCE_REGISTER: &[u8] = &[
        // mov64 r0, r12  (opcode=0xbf, dst=0, src=12, off=0, imm=0)
        0xbf, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // exit
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    /// UnknownOpCode: First byte is 0x06 which is not a valid opcode.
    pub const UNKNOWN_OPCODE: &[u8] = &[
        // unknown opcode 0x06
        0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // exit
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    /// InfiniteLoop: `ja +(-1)` — jumps to itself.
    /// The verifier detects `offset == -1` as an infinite loop.
    pub const INFINITE_LOOP: &[u8] = &[
        // ja -1  (opcode=0x05, dst=0, src=0, off=-1, imm=0)
        0x05, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
        // exit (unreachable, but needed for program length)
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    /// Helper to build an instruction from its fields.
    pub fn make_insn(opc: u8, dst: u8, src: u8, off: i16, imm: i32) -> [u8; 8] {
        let mut buf = [0u8; ebpf::INSN_SIZE];
        buf[0] = opc;
        buf[1] = (src << 4) | (dst & 0x0f);
        buf[2..4].copy_from_slice(&off.to_le_bytes());
        buf[4..8].copy_from_slice(&imm.to_le_bytes());
        buf
    }

    /// Build a raw program from a slice of 8-byte instructions.
    pub fn program_from_insns(insns: &[[u8; 8]]) -> Vec<u8> {
        insns.iter().flat_map(|i| i.iter().copied()).collect()
    }
}
