//! The program ELFs to benchmark against.

use {
    solana_sbpf::{elf::get_sbpf_version, program::SBPFVersion},
    std::{
        env, fs,
        path::{Path, PathBuf},
    },
};

pub const RELATIVE_CALL_SBPFV0: &[u8] = include_bytes!("../../tests/elfs/relative_call_sbpfv0.so");
pub const RELATIVE_CALL: &[u8] = include_bytes!("../../tests/elfs/relative_call.so");

/// Points at a directory of cluster programs dumped by
/// `solana program dump <PROGRAM_ID> <FILE>`, sorted into `v0` and `v3`
/// subdirectories by SBPF version.
const PROGRAMS_DIR_VAR: &str = "SBPF_BENCH_PROGRAMS_DIR";

pub struct Program {
    pub name: String,
    pub elf: Vec<u8>,
    /// Read back out of the ELF header, since that is what selects the loading
    /// path rather than the directory the program was found in.
    pub sbpf_version: SBPFVersion,
}

impl Program {
    fn new(name: String, elf: Vec<u8>) -> Self {
        let sbpf_version = get_sbpf_version(&elf)
            .unwrap_or_else(|err| panic!("cannot read the SBPF version of {}: {}", name, err));
        Self {
            name,
            elf,
            sbpf_version,
        }
    }

    /// Whether this program is loaded by the strict parser.
    pub fn is_strict(&self) -> bool {
        self.sbpf_version.enable_stricter_elf_headers()
    }
}

fn collect(dir: &Path, prefix: &str, programs: &mut Vec<Program>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("cannot read directory {}: {}", dir.display(), err));
    for entry in entries {
        let path = entry.expect("cannot read directory entry").path();
        let stem = path
            .file_stem()
            .expect("directory entry has a file stem")
            .to_string_lossy();
        if path.is_dir() {
            collect(&path, &format!("{prefix}{stem}_"), programs);
        } else if path.extension().is_some_and(|extension| extension == "so") {
            let name = format!("{prefix}{stem}");
            let elf =
                fs::read(&path).unwrap_or_else(|err| panic!("cannot read {}: {}", name, err));
            programs.push(Program::new(name, elf));
        }
    }
}

/// The in-tree ELFs, plus every `*.so` under the programs directory. Corpus
/// programs are named after their path relative to that directory, so
/// `v0/<program_id>.so` is named `v0_<program_id>`.
pub fn programs() -> Vec<Program> {
    let mut programs = vec![
        Program::new(
            "relative_call_sbpfv0".to_string(),
            RELATIVE_CALL_SBPFV0.to_vec(),
        ),
        Program::new("relative_call".to_string(), RELATIVE_CALL.to_vec()),
    ];

    let Some(dir) = env::var_os(PROGRAMS_DIR_VAR).map(PathBuf::from) else {
        eprintln!(
            "{PROGRAMS_DIR_VAR} is unset, benchmarking the in-tree ELFs only. Point it at a \
             directory of program ELFs.",
        );
        return programs;
    };

    let mut corpus = Vec::new();
    collect(&dir, "", &mut corpus);
    // Sorted so that benchmark ids are stable from run to run.
    corpus.sort_by(|left, right| left.name.cmp(&right.name));
    programs.append(&mut corpus);
    programs
}
