#![allow(unexpected_cfgs)]

#[allow(unused_imports)]
pub use solana_program_stubs;

#[macro_export]
#[cfg(not(target_os = "solana"))]
macro_rules! coverage {
    ($entry_function:expr) => {
        $crate::solana_program_stubs::declare_sol_app_stubs!($entry_function);
    };
}

#[macro_export]
#[cfg(not(target_os = "solana"))]
macro_rules! anchor_coverage {
    () => {
        $crate::solana_program_stubs::declare_sol_app_stubs!(entry);
    };
}

#[macro_export]
#[cfg(not(target_os = "solana"))]
macro_rules! coveragev2 {
    ($entry_function:expr) => {
        $crate::solana_program_stubs::declare_sol_app_stubsv2!($entry_function);
    };
}

#[macro_export]
#[cfg(not(target_os = "solana"))]
macro_rules! anchor_coveragev2 {
    () => {
        $crate::solana_program_stubs::declare_sol_app_stubsv2!(entry);
    };
}
