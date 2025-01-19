//! Check that a signature that *does* hold *doesn't* fail.

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
fn subset(x: Positive<u8>) -> NonNegative<u8> {
    x.also()
}
