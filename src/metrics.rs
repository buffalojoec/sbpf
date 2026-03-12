//! Telemetry structs for load and verify phases.

/// Metrics collected during ELF loading.
#[derive(Debug, Default)]
pub struct LoadMetrics {
    /// Time in microseconds for `Elf64::parse()`
    pub parse_us: u64,
    /// Time in microseconds for `validate()`
    pub validate_us: u64,
    /// Time in microseconds for `relocate()`
    pub relocate_us: u64,
    /// Time in microseconds for `parse_ro_sections()`
    pub ro_sections_us: u64,
    /// Time in microseconds for entry point + function registry setup
    pub entry_us: u64,
}

/// Metrics collected during bytecode verification.
#[derive(Debug, Default)]
pub struct VerifyMetrics {
    /// Number of instructions in the program
    pub instruction_count: u64,
}
