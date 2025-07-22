# solana-coverage

This crate provides a macro for setting stubs necessary for obtaining
code coverage from an Anchor program.

## Usage

```
// Place in the Anchor program's lib.rs
#[cfg(not(target_os = "solana"))]
mod coverage {
    use super::*;
    use anchor_lang::solana_program::program_stubs::{set_syscall_stubs, SyscallStubs};
    solana_coverage::anchor_coverage!();
}

#[program]
pub mod my_anchor_program {
...
}
```

## License

This project is licensed under the [MIT License](LICENSE).

---

© 2025 LimeChain
