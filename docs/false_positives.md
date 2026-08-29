# Handling False Positives

Static analysis tools occasionally flag code that is intentionally written the way it is. This guide explains how to recognize, suppress, and report false positives in `soroban-cost-linter`.

## What is a False Positive?

A false positive is a lint warning that fires on code that does not actually contain the problem the lint is designed to catch.

For example, `soroban_storage_in_loop` warns when a storage operation appears inside a loop body. In most code this is an expensive anti-pattern, but if you are intentionally writing different keys on each iteration (e.g., writing a batch of entries), the warning is a false positive — the code is correct, and the cost is inherent to the operation.

## Known False Positive Patterns by Lint

### `signature_verification_in_loop`

This lint flags elliptic-curve cryptographic signature verification calls (`ed25519_verify`, `secp256k1_recover`, `secp256r1_verify`) inside loop bodies. False positives or accepted patterns include:

- **Fixed small multi-signature sets** — verifying a fixed, small, compile-time-known set of signatures (e.g., a 3-of-5 multisig check with unrolled or small fixed iteration counts) where a batch API is unavailable or inapplicable.
- **Batching unavailable** — when distinct signers and messages require independent verification and no native batch verification primitive exists for the specific combination.

### `soroban_storage_in_loop`

Every storage read or write inside any loop body is flagged. This is correct for the dominant case, but false positives arise when:

- **Batch writes with different keys** — iterating over a collection and writing each element under a different storage key.
- **Storage reads that depend on the loop variable** — reading a value for each item in a collection, where the key changes per iteration.
- **Counting or scanning patterns** — using a loop to count entries or scan through storage with `has()`.

The lint does not analyse whether the key changes between iterations; it errs on the side of reporting.

### `unnecessary_host_function_call`

This lint uses mutation analysis to leave calls alone when their arguments depend on loop state. Known gaps that produce false positives:

- **Bindings and mutations inside a closure body** nested in the loop are not tracked.
- **Mutation through a raw pointer or interior mutability** (`Cell`, `RefCell`) is not tracked.
- **Intentional per-iteration calls** like `env.prng().u64_in_range()` or `env.events().publish()` with constant arguments are still reported — the lint cannot distinguish intent from waste.

### `redundant_env_clone`

This lint fires for every `.clone()` call on `Env`. False positives occur when:

- The `Env` is consumed before the clone site and you genuinely need a second handle.
- The code is generic over a trait that does not guarantee `Env`-like cheap pass-by-value semantics.

### `symbol_new_for_short_literal`

This lint fires when `Symbol::new(&env, literal)` is called with a short literal. False positives occur when:

- The literal is constructed dynamically (non-literal argument) — the lint already handles this.
- The macro `symbol_short!` is unavailable in your environment (e.g., an older SDK version).

### `unbounded_recursion`

This lint flags a recursive call cycle (direct or mutual) whose depth is driven by
caller-supplied input — a caller-supplied `Vec`/`&[T]` length, a tail slice, or a
slicing/`to_vec` operation on caller data. False positives and accepted gaps:

- **Structurally-bounded recursion reported as unbounded:** a collection consumed by a method *not* in the recognized tail set (e.g. a custom `fn rest(&self) -> Self` returning a strict sub-slice) may not be recognized as progress. Prefer `#[allow(unbounded_recursion)]` for such intentional, provably-bounded cases.
- **Constant-argument "infinite-looking" recursion:** `fn f(n: u32) { if n == 0 { return; } f(3); }` passes a constant argument, so the lint treats it as bounded and stays silent even though `n` never decreases. The lint keys off the *argument shape*, not the actual termination proof, to stay sound and simple.
- **Plain integer parameters threaded through the recursion:** `fn process(n: u32) { if n == 0 { return; } process(n - 1); }` is structurally a countdown, but the *initial* value of `n` is caller-supplied, so the depth is not provably constant.
