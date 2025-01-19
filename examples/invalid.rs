//! Check that a signature that *doesn't* hold *does* fail.

#![expect(
    unused_crate_dependencies,
    reason = "not al examples use all dependencies"
)]

use {proc_macro2 as _, quote as _, sigma_types::Positive, sigma_types_macros::forall, syn as _};

fn main() {}

/// Inverse of the successor operation.
/// Where defined, `\x. x - 1`.
#[forall]
#[expect(dead_code, reason = "macro fodder")]
#[expect(clippy::arithmetic_side_effects, reason = "that's the point")]
fn predecessor(x: Positive<u8>) -> Positive<u8> {
    let y = x.get() - 1;
    Positive::new(y)
}
