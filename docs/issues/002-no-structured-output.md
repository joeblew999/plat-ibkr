# Issue: No Structured Output Format in Examples

**Repository:** [wboayue/rust-ibapi](https://github.com/wboayue/rust-ibapi)

**Type:** Enhancement

## Problem

Examples output Rust debug format (`{:?}`) which is not easily parseable by other tools like `jq`, `csvtool`, or shell scripts.

## Current Behavior

```rust
// examples/sync/historical_data.rs
for bar in &subscription {
    println!("{bar:?}")
}
```

**Output:**
```
Bar { date: 2023-04-11 13:30:00.0 +00:00:00, open: 162.35, high: 162.36, low: 161.08, close: 161.23, volume: 54017.51, wap: 161.637, count: 24546 }
```

This format:
- Cannot be piped to `jq`
- Cannot be imported into spreadsheets
- Requires custom parsing

## Suggested Fix

### Add `--format` flag to examples

```rust
use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    #[arg(short, long, value_enum, default_value = "text")]
    format: OutputFormat,
}

#[derive(Copy, Clone, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}
```

### JSON Output

```rust
match args.format {
    OutputFormat::Json => {
        println!("{}", serde_json::to_string(&bar)?);
    }
    // ...
}
```

**Result:**
```json
{"date":"2023-04-11T13:30:00Z","open":162.35,"high":162.36,"low":161.08,"close":161.23,"volume":54017.51}
```

### CSV Output

```rust
OutputFormat::Csv => {
    println!("{},{},{},{},{},{}", bar.date, bar.open, bar.high, bar.low, bar.close, bar.volume);
}
```

## Use Cases

- Pipe historical data to analysis tools: `cargo run --example historical_data -- --format json | jq '.close'`
- Export to spreadsheet: `cargo run --example historical_data -- --format csv > data.csv`
- Integration with trading bots and automation scripts

## Note

The data structures already derive `Serialize`, so JSON/CSV output is straightforward to implement.
