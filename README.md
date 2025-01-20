# Macros to enhance the `sigma-types` crate

## Universal type safety

The `#[forall]` macro automatically writes a test that ensures that,
for *any* possible inputs, a function never panics
(which would indicate a broken sigma-type guarantee).
This helps enforce a consistent paradigm in which
any "invariants" or special caveats ought to be readable
directly off an item's type signature for its end-users:
if a certain input combination causes a function to fail,
then it ought to be explicitly disallowed at the type level.
(Internally, this uses `quicheck` to do most of the heavy lifting.)
