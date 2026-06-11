//! Abort shim for the Fortanix SGX target.
//!
//! Why this exists:
//!
//! The bundled `wasm3` runtime (via `wasm3-sys`) invokes the standard C
//! function `abort()` from native code (`m3_core.c` -> `m3_Abort`).
//! The Fortanix SGX environment does not provide a libc implementation
//! of `abort`, which can result in linker errors such as:
//!
//!     undefined symbol: abort
//!
//! This shim exports a C-compatible symbol named `abort` and forwards
//! execution to Rust's internal abort routine, ensuring that fatal runtime
//! errors terminate execution immediately and never return.

#[no_mangle]
pub extern "C" fn abort() -> ! {
    extern "C" {
        fn __rust_abort() -> !;
    }

    unsafe { __rust_abort() }
}
