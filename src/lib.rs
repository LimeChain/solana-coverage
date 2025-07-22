#![allow(unexpected_cfgs)]

#[allow(unused_imports)]
pub use solana_program_stubs;

/// A macro providing the necessary stubs for a Solana program.
#[macro_export]
#[cfg(not(target_os = "solana"))]
macro_rules! anchor_coverage {
    () => {
        $crate::solana_program_stubs::declare_sol_app_stubs!();
    };
}
