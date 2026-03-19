//! Minimal ELF builder for verification testing.
//!
//! Produces ELF bytes that pass [`Executable::load()`]'s structural checks
//! so that we can exercise the verifier (or skip it) and then invoke the
//! loaded program.
//!
//! Two entry-points are provided:
//!
//! * [`elf_from_text_bytes`] — wraps raw instruction bytes in a strict-format
//!   ELF (V3 program headers layout).
//! * [`elf_with_assembly`] — assembles an instruction string first, then wraps
//!   the result.

use solana_sbpf::{
    assembler::assemble,
    ebpf,
    program::{BuiltinProgram, SBPFVersion},
    vm::{Config, ContextObject},
};
use std::sync::Arc;

// ---- ELF layout constants (strict parser, V3+) ---------------------------
//
// Layout:
//   [Elf64Ehdr]            64 bytes   offset 0x00
//   [Elf64Phdr] rodata     56 bytes   offset 0x40
//   [Elf64Phdr] text       56 bytes   offset 0x78
//   <rodata bytes>          0 bytes   offset 0xB0  (empty, but correctly sized)
//   <text bytes>            N bytes   offset 0xB0
//
// Total header region = 64 + 56*2 = 176 = 0xB0

const EHDR_SIZE: usize = 64;
const PHDR_SIZE: usize = 56;
const HEADER_REGION: usize = EHDR_SIZE + 2 * PHDR_SIZE; // 0xB0 = 176

/// Build a minimal V3-strict ELF from raw text (instruction) bytes.
///
/// The resulting bytes can be loaded with [`Executable::load()`] when a
/// loader with `enabled_sbpf_versions` containing V3 is provided.
///
/// `text_bytes` must be a multiple of 8 bytes (INSN_SIZE) and non-empty.
pub fn elf_from_text_bytes(text_bytes: &[u8]) -> Vec<u8> {
    assert!(
        !text_bytes.is_empty() && text_bytes.len().is_multiple_of(ebpf::INSN_SIZE),
        "text_bytes must be non-empty and a multiple of {} bytes",
        ebpf::INSN_SIZE,
    );

    // The text segment vaddr must be MM_BYTECODE_START (0x1_0000_0000).
    // The entrypoint is the first instruction of the text segment.
    let text_vaddr: u64 = ebpf::MM_BYTECODE_START;
    let text_offset = HEADER_REGION as u64;
    let text_size = text_bytes.len() as u64;

    // Rodata: empty segment immediately before text, at vaddr 0.
    let rodata_offset = HEADER_REGION as u64;
    let rodata_vaddr: u64 = ebpf::MM_RODATA_START;
    let rodata_size: u64 = 0;

    let mut buf = Vec::with_capacity(HEADER_REGION + text_bytes.len());

    // --- ELF header (64 bytes) -------------------------------------------
    // e_ident (16 bytes)
    buf.extend_from_slice(&[0x7f, 0x45, 0x4c, 0x46]); // EI_MAG
    buf.push(2); // EI_CLASS = ELFCLASS64
    buf.push(1); // EI_DATA  = ELFDATA2LSB
    buf.push(1); // EI_VERSION = EV_CURRENT
    buf.push(0); // EI_OSABI = ELFOSABI_NONE
    buf.push(0); // EI_ABIVERSION
    buf.extend_from_slice(&[0u8; 7]); // EI_PAD
                                      // e_type (2 bytes) — doesn't matter for strict parser (not checked)
    buf.extend_from_slice(&3u16.to_le_bytes()); // ET_DYN
                                                // e_machine (2 bytes)
    buf.extend_from_slice(&247u16.to_le_bytes()); // EM_BPF
                                                  // e_version (4 bytes)
    buf.extend_from_slice(&1u32.to_le_bytes()); // EV_CURRENT
                                                // e_entry (8 bytes) — entrypoint vaddr, must be within text segment
    buf.extend_from_slice(&text_vaddr.to_le_bytes());
    // e_phoff (8 bytes) — program header table offset = sizeof(Ehdr)
    buf.extend_from_slice(&(EHDR_SIZE as u64).to_le_bytes());
    // e_shoff (8 bytes) — section header table offset (0 = none)
    buf.extend_from_slice(&0u64.to_le_bytes());
    // e_flags (4 bytes) — SBPF version 3
    buf.extend_from_slice(&3u32.to_le_bytes());
    // e_ehsize (2 bytes)
    buf.extend_from_slice(&(EHDR_SIZE as u16).to_le_bytes());
    // e_phentsize (2 bytes)
    buf.extend_from_slice(&(PHDR_SIZE as u16).to_le_bytes());
    // e_phnum (2 bytes) — 2 program headers (rodata + text)
    buf.extend_from_slice(&2u16.to_le_bytes());
    // e_shentsize (2 bytes)
    buf.extend_from_slice(&0u16.to_le_bytes());
    // e_shnum (2 bytes)
    buf.extend_from_slice(&0u16.to_le_bytes());
    // e_shstrndx (2 bytes)
    buf.extend_from_slice(&0u16.to_le_bytes());
    debug_assert_eq!(buf.len(), EHDR_SIZE);

    // --- Program header 0: rodata (PF_R) ---------------------------------
    write_phdr(
        &mut buf,
        1, // PT_LOAD
        4, // PF_R
        rodata_offset,
        rodata_vaddr,
        rodata_size,
    );

    // --- Program header 1: text (PF_X) -----------------------------------
    write_phdr(
        &mut buf,
        1, // PT_LOAD
        1, // PF_X
        text_offset,
        text_vaddr,
        text_size,
    );
    debug_assert_eq!(buf.len(), HEADER_REGION);

    // --- Text bytes (the actual instructions) ----------------------------
    buf.extend_from_slice(text_bytes);

    buf
}

/// Assemble an instruction string and wrap the result in a V3 ELF.
///
/// The assembly is performed using the sbpf assembler with a mock loader
/// configured for `sbpf_version`. The resulting instruction bytes are then
/// wrapped in a minimal ELF.
pub fn elf_with_assembly<C: ContextObject>(asm: &str, sbpf_version: SBPFVersion) -> Vec<u8> {
    let config = Config {
        enabled_sbpf_versions: SBPFVersion::V0..=sbpf_version,
        ..Config::default()
    };
    let loader = Arc::new(BuiltinProgram::new_loader(config));
    let executable = assemble::<C>(asm, loader).expect("assembly should succeed");
    let (_vaddr, text_bytes) = executable.get_text_bytes();
    elf_from_text_bytes(text_bytes)
}

fn write_phdr(
    buf: &mut Vec<u8>,
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_filesz: u64,
) {
    buf.extend_from_slice(&p_type.to_le_bytes()); // p_type
    buf.extend_from_slice(&p_flags.to_le_bytes()); // p_flags
    buf.extend_from_slice(&p_offset.to_le_bytes()); // p_offset
    buf.extend_from_slice(&p_vaddr.to_le_bytes()); // p_vaddr
    buf.extend_from_slice(&p_vaddr.to_le_bytes()); // p_paddr = p_vaddr
    buf.extend_from_slice(&p_filesz.to_le_bytes()); // p_filesz
    buf.extend_from_slice(&p_filesz.to_le_bytes()); // p_memsz = p_filesz
    buf.extend_from_slice(&0u64.to_le_bytes()); // p_align
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestContextObject;
    use solana_sbpf::{elf::Executable, verifier::RequisiteVerifier};

    #[test]
    fn valid_elf_loads_and_verifies() {
        // A trivial valid program: `mov32 r0, 0` + `exit`
        let elf = elf_with_assembly::<TestContextObject>("mov32 r0, 0\nexit", SBPFVersion::V3);
        let config = Config {
            enabled_sbpf_versions: SBPFVersion::V3..=SBPFVersion::V3,
            ..Config::default()
        };
        let loader = Arc::new(BuiltinProgram::new_loader(config));
        let executable =
            Executable::<TestContextObject>::load(&elf, loader).expect("ELF load should succeed");
        executable
            .verify::<RequisiteVerifier>()
            .expect("verification should pass");
    }

    #[test]
    fn invalid_elf_loads_but_fails_verification() {
        // Division by zero — loads fine structurally, fails verification.
        let elf = elf_with_assembly::<TestContextObject>("div32 r0, 0\nexit", SBPFVersion::V3);
        let config = Config {
            enabled_sbpf_versions: SBPFVersion::V3..=SBPFVersion::V3,
            ..Config::default()
        };
        let loader = Arc::new(BuiltinProgram::new_loader(config));
        let executable =
            Executable::<TestContextObject>::load(&elf, loader).expect("ELF load should succeed");
        let err = executable.verify::<RequisiteVerifier>().unwrap_err();
        assert!(
            format!("{err:?}").contains("DivisionByZero"),
            "expected DivisionByZero, got: {err:?}",
            err = err,
        );
    }

    #[test]
    fn raw_bytes_elf_loads() {
        use crate::corpus::raw;
        let elf = elf_from_text_bytes(raw::UNKNOWN_OPCODE);
        let config = Config {
            enabled_sbpf_versions: SBPFVersion::V3..=SBPFVersion::V3,
            ..Config::default()
        };
        let loader = Arc::new(BuiltinProgram::new_loader(config));
        let executable =
            Executable::<TestContextObject>::load(&elf, loader).expect("ELF load should succeed");
        let err = executable.verify::<RequisiteVerifier>().unwrap_err();
        assert!(
            format!("{err:?}").contains("UnknownOpCode"),
            "expected UnknownOpCode, got: {err:?}",
            err = err,
        );
    }
}
