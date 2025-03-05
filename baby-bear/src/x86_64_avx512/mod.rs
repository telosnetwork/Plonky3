// Compile-time check to force the use of a nightly toolchain from 2024-10-29 or later.
#[rustversion::before(2024-10-28)]
fn compile_time_version_check() {
    compile_error!("AVX-512 requires a nightly toolchain from 2024-10-29 or later");
}

mod packing;
mod poseidon2;

pub use packing::*;
