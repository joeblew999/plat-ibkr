# Issue: Missing CLI Arguments in Some Examples

**Repository:** [wboayue/rust-ibapi](https://github.com/wboayue/rust-ibapi)

**Type:** Enhancement

## Problem

Some examples require code modifications to change parameters like symbols, date ranges, or account IDs. This makes testing different scenarios cumbersome.

## Examples Affected

### `market_data.rs`
```rust
// Hardcoded symbol
let contract = Contract::stock("AAPL").build();
```

### `account_summary.rs`
```rust
// No way to filter by account
let subscription = client.account_summary(&AccountGroup("All".to_string()), tags)?;
```

### `historical_data.rs`
Good - already accepts symbol as CLI arg:
```rust
let symbol = &args[1];
```

But missing:
- Date range arguments
- Bar size argument
- What to show (TRADES, MIDPOINT, BID, ASK)

## Suggested Fix

### Consistent CLI Interface

All examples should use `clap` for argument parsing:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "market_data")]
#[command(about = "Stream real-time market data")]
struct Args {
    /// Stock symbol
    symbol: String,

    /// Exchange (optional)
    #[arg(short, long)]
    exchange: Option<String>,

    /// Currency (default: USD)
    #[arg(short, long, default_value = "USD")]
    currency: String,
}
```

### Example Usage After Fix

```bash
# Market data
cargo run --example market_data -- TSLA

# Historical data with date range
cargo run --example historical_data -- AAPL --start 2024-01-01 --end 2024-01-31

# Account summary for specific account
cargo run --example account_summary -- --account DU123456
```

## Benefits

- No code changes needed to test different scenarios
- Scriptable/automatable
- Self-documenting via `--help`
- Consistent UX across all examples
