//! Compiles `dsp/moodscore.dsp` to Rust with the Faust compiler.
//!
//! - If `faust` is available (on `PATH`, or at the path in the `FAUST`
//!   environment variable), the DSP is compiled fresh on every change.
//! - If it is not, the pre-generated code in `generated/` is used, so the
//!   project builds without Faust installed.
//! - The generated code must not take locks: Faust guards shared lookup
//!   tables (`rdtable`, table-based oscillators such as `os.osc`) with a
//!   `RwLock` read in every `compute()` call. The build fails if that happens.
//! - Set `MOODSCORE_UPDATE_GENERATED=1` to refresh `generated/` from a fresh
//!   compile. Do this (and commit the result) whenever `dsp/` changes.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const CLASS_NAME: &str = "MoodscoreDsp";
const FILE_NAME: &str = "moodscore_dsp.rs";

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dsp_dir = manifest_dir.join("../../dsp");
    let dsp_file = dsp_dir.join("moodscore.dsp");
    let committed = manifest_dir.join("generated").join(FILE_NAME);
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join(FILE_NAME);

    println!("cargo:rerun-if-changed={}", dsp_dir.display());
    println!("cargo:rerun-if-changed={}", committed.display());
    println!("cargo:rerun-if-env-changed=FAUST");
    println!("cargo:rerun-if-env-changed=MOODSCORE_UPDATE_GENERATED");

    let faust = env::var("FAUST").unwrap_or_else(|_| "faust".to_string());
    let result = Command::new(&faust)
        .arg("-lang")
        .arg("rust")
        .arg("-cn")
        .arg(CLASS_NAME)
        .arg("-I")
        .arg(&dsp_dir)
        .arg(&dsp_file)
        .arg("-o")
        .arg(&out)
        .status();

    match result {
        Ok(status) if status.success() => {
            check_real_time_safe(&out, &dsp_file);
            if env::var_os("MOODSCORE_UPDATE_GENERATED").is_some() {
                fs::copy(&out, &committed).expect("failed to update generated/ DSP code");
                println!(
                    "cargo:warning=Updated {} from {}",
                    committed.display(),
                    dsp_file.display()
                );
            }
        }
        Ok(status) => {
            panic!("Faust failed to compile {} ({status})", dsp_file.display());
        }
        Err(_) => {
            fs::copy(&committed, &out).unwrap_or_else(|e| {
                panic!(
                    "Faust compiler not found and no pre-generated code at {} ({e})",
                    committed.display()
                )
            });
            println!(
                "cargo:warning=Faust not found: using pre-generated DSP code. \
                 Install Faust to rebuild the DSP from dsp/."
            );
        }
    }
}

/// Fail the build if the generated DSP would lock inside `compute()`.
fn check_real_time_safe(generated: &std::path::Path, dsp_file: &std::path::Path) {
    let code = fs::read_to_string(generated).expect("failed to read generated DSP code");
    if code.contains("RwLock") {
        panic!(
            "{} compiles to code that takes a lock in compute() (a Faust lookup \
             table such as rdtable or os.osc). The audio thread must never lock: \
             use a table-free alternative (e.g. os.oscs instead of os.osc).",
            dsp_file.display()
        );
    }
}
