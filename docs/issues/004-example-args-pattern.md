# Issue: Standardize Example Arguments Pattern

**Repository:** [wboayue/rust-ibapi](https://github.com/wboayue/rust-ibapi)

**Type:** Enhancement

## Problem

Examples need a consistent, simple way to accept arguments without adding heavy dependencies or complex code that obscures the actual API usage being demonstrated.

## Goals

1. **Keep examples simple** - The focus should be on demonstrating the ibapi library
2. **Allow customization** - Users should be able to change symbols, accounts, etc. without editing code
3. **No heavy dependencies** - Avoid adding `clap` or other large crates to examples
4. **Consistent pattern** - Same approach across all examples

## Suggested Pattern: Environment Variables + Simple Args

### Pattern 1: Environment Variables (Recommended for Examples)

```rust
//! Market Data Example
//!
//! # Usage
//! ```bash
//! # Default (AAPL)
//! cargo run --example market_data
//!
//! # Custom symbol
//! SYMBOL=TSLA cargo run --example market_data
//!
//! # Custom connection
//! IBKR_HOST=192.168.1.100 IBKR_PORT=4001 cargo run --example market_data
//! ```

use std::env;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Connection settings (consistent across all examples)
    let host = env::var("IBKR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("IBKR_PORT").unwrap_or_else(|_| "4002".to_string()).parse()?;
    let client_id: i32 = env::var("IBKR_CLIENT_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::process::id() as i32); // Use PID for uniqueness

    // Example-specific settings
    let symbol = env::var("SYMBOL").unwrap_or_else(|_| "AAPL".to_string());

    let client = Client::connect(&format!("{}:{}", host, port), client_id)?;

    // ... rest of example
}
```

### Pattern 2: Positional Args for Required Values

For examples where a value is commonly changed (like symbol):

```rust
//! Historical Data Example
//!
//! # Usage
//! ```bash
//! cargo run --example historical_data AAPL
//! cargo run --example historical_data TSLA
//! ```

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let symbol = args.get(1)
        .map(|s| s.as_str())
        .unwrap_or("AAPL");

    // ... rest of example
}
```

### Pattern 3: Combined (Best of Both)

```rust
//! Account Summary Example
//!
//! # Environment Variables
//! - `IBKR_HOST` - Gateway host (default: 127.0.0.1)
//! - `IBKR_PORT` - Gateway port (default: 4002)
//! - `IBKR_CLIENT_ID` - Client ID (default: random)
//! - `OUTPUT` - Output format: text, json, csv (default: text)
//!
//! # Usage
//! ```bash
//! cargo run --example account_summary
//! OUTPUT=json cargo run --example account_summary
//! OUTPUT=csv cargo run --example account_summary > account.csv
//! ```

use std::env;

fn main() -> anyhow::Result<()> {
    // Standard connection setup (copy-paste across examples)
    let host = env::var("IBKR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("IBKR_PORT").unwrap_or_else(|_| "4002".to_string());
    let client_id: i32 = env::var("IBKR_CLIENT_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::process::id() as i32);

    let output = env::var("OUTPUT").unwrap_or_else(|_| "text".to_string());

    let client = Client::connect(&format!("{}:{}", host, port), client_id)?;

    // ... example code ...

    // Output based on format
    match output.as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&data)?),
        "csv" => print_csv(&data),
        _ => print_text(&data),
    }

    Ok(())
}
```

## Recommended Standard Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `IBKR_HOST` | 127.0.0.1 | TWS/Gateway host |
| `IBKR_PORT` | 4002 | TWS/Gateway port (4002=paper, 4001=live) |
| `IBKR_CLIENT_ID` | PID | Unique client ID |
| `SYMBOL` | AAPL | Stock symbol (where applicable) |
| `OUTPUT` | text | Output format (text/json/csv) |
| `ACCOUNT` | (all) | Account filter (where applicable) |

## Benefits

1. **Zero dependencies** - Uses only `std::env`
2. **Self-documenting** - Doc comments show all options
3. **Composable** - Can combine env vars with positional args
4. **Shell-friendly** - Easy to script: `for s in AAPL TSLA MSFT; do SYMBOL=$s cargo run --example market_data; done`
5. **Consistent** - Same pattern across all 70+ examples

## Migration Path

1. Add env var support to connection setup in all examples
2. Add `OUTPUT` env var for structured output
3. Document in example header comments
4. Keep existing positional args where they exist
