# Rust unsafe inventory

This document lists every `unsafe` block, function, and trait implementation in
the two Rust workspaces (`ffi/rust`, `platforms/web/server/precompute`), and
states for each one why it exists and which obligations the caller carries.
Occurrences of the word `unsafe` that are not unsafe code (`unsafe_break_count`
fields, the `unsafeBreakCount` JSON key, the `unsafe_href` function, test
names) are out of scope.

The `unsafe` code sits on one remaining boundary: the shared-library lifecycle
boundary between the Neon addon and the host node process. The former C ABI
boundary (protocol in `ffi/native/tiqian_layout_abi.h`) disappeared with the
`ffi/native` module; the engine is consumed directly through the generated
Rust types (ADR 0050). Code outside the remaining boundary is safe Rust.

## engine_bridge.rs: session-to-engine borrow boundary

File `platforms/web/server/precompute/engine/src/engine_bridge.rs`.

The generated engine receives the font session as generated trait objects
(`ITextShaper`, `FontMetricsResolver`); there is no C ABI left on this
boundary. One `unsafe` site remains, and it exists because the generated
trait objects require a type with `'static` lifetime while the session and
its capture window are borrows that live exactly as long as one
`precompute_paragraph` call.

| Site | Form | Why it exists |
| --- | --- | --- |
| `SessionBackend::new` | `ptr::from_ref` / `ptr::from_mut` | erases the borrows into raw pointers so two `Box<dyn ...>` component slots can share one session and one capture window |
| `with_access` | `&*self.session`, `&mut *self.evidence` | rebuilds the references for each component callback |
| `unsafe impl Send / Sync for SessionBackend` | trait impls | satisfies the generated `Send + Sync` bounds on `ITextShaper`; no cross-thread use happens |

Invariants (the obligations the caller, `precompute_paragraph`, carries): the
`&FontSession` and `&mut CaptureEvidence` borrows outlive the `Engine` value,
so both pointers stay valid while the engine can reach them; the engine runs
its layout synchronously on the calling thread, so all access through the
pointers is sequential and never re-entrant; the aliasing `&mut` rebuild is
sound because only one component callback runs at a time.

## pin.rs: AddonMappingPin

File `platforms/web/server/precompute/binding/src/pin.rs`.

Background: node calls `dlclose` on every addon an environment loaded when
that environment is destroyed, which happens when a worker thread exits. An
engine call attaches the Kotlin/Native runtime to the calling thread and
registers thread-local destructors inside this library; the runtime also
starts its GC threads, which belong to the process. After the unmap, those
destructors and threads point at unmapped code, and `__nptl_deallocate_tsd`
raises SIGSEGV during worker teardown. The vite SSR worker pool reproduces
this crash: the build succeeds, and the process crashes on exit.

| Site | Form | Why it exists |
| --- | --- | --- |
| `pin_once` / `pin` on unix | `dladdr`, `dlopen` declarations and calls | `dladdr` locates the shared library that contains this function; `dlopen` on a file that is already loaded raises the reference count and marks the library NODELETE without running initializers again |
| `pin` on windows | `GetModuleHandleExW` declaration and call | the PIN flag ties the DLL to the process lifetime, so a later `FreeLibrary` does not unload it |

Invariants: `pin_once` runs once, at the addon registration entry; the handle
from `dlopen` is never closed and holds the reference for the process
lifetime; a failed call leaves the library unpinned, which matches the
behavior before the fix. The unix path is verified against this crash; the
windows and arm64 paths compile, and no run in this repository exercises them.

## Residual risk

- A Rust panic that unwinds through a C callback into Kotlin frames is
  undefined behavior. The panic sources inside the callbacks are allocation
  failure and the `unwrap_or` fallbacks; session callbacks report every error
  through the error code path.
- Leak paths: the plan and error buffers are released in pairs, and the
  error strings change hands in pairs. No leak path across the boundary is
  known.
- The registry of `tiqian-precompute-neon` keeps its entries addressable
  after close. This is not unsafe code and not a leak; memory grows when a
  process creates precomputers in a loop. This is a trade-off.