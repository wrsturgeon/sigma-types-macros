//! Check that a signature with *references*
//! gets converted into a format that `quickcheck` can understand.

#![expect(
    unused_crate_dependencies,
    reason = "not al examples use all dependencies"
)]

use {
    proc_macro2 as _, quote as _,
    sigma_types::{NonNegative, Positive},
    sigma_types_macros::forall,
    syn as _,
};

fn main() {}

/// Trivially convert a positive number into a non-negative number.
#[forall]
#[expect(dead_code, reason = "macro fodder")]
#[expect(clippy::trivially_copy_pass_by_ref, reason = "macro test")]
fn subset(x: &Positive<u8>) -> NonNegative<u8> {
    let y = *x;
    y.also()
}
