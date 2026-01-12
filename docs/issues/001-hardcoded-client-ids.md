# Issue: Hardcoded Client IDs in Examples

**Repository:** [wboayue/rust-ibapi](https://github.com/wboayue/rust-ibapi)

**Type:** Enhancement

## Problem

Examples use hardcoded client IDs (typically `100`), which causes connection failures when:
- Running multiple examples sequentially
- Running examples while another client is connected
- Testing in development environments

**Error message:**
```
Unable to connect as the client id is already in use. Retry with a unique client id.
```

## Current Behavior

```rust
// examples/sync/account_summary.rs
let client = Client::connect("127.0.0.1:4002", 100)?;
```

All examples use the same client ID, causing conflicts.

## Suggested Fix

### Option 1: Use Process ID (Recommended - No Dependencies)

```rust
fn main() -> anyhow::Result<()> {
    let client_id = std::process::id() as i32;
    let client = Client::connect("127.0.0.1:4002", client_id)?;
    // ...
}
```

Simple, unique per process, no new dependencies.

### Option 2: Environment Variable Override

```rust
let client_id: i32 = std::env::var("IBKR_CLIENT_ID")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or_else(|| std::process::id() as i32);
```

Allows override when needed, defaults to PID.

## Impact

- Improves developer experience
- Allows running examples without manual code changes
- Enables automated testing of examples
- Zero additional dependencies
