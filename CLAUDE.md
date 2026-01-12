# CLAUDE.md

IBKR stocks trading platform using rust-ibapi (ibapi crate).

**See xplat docs:** https://raw.githubusercontent.com/joeblew999/xplat/main/CLAUDE.md

## Quick Start

```sh
xplat task build      # Build the project
xplat task run        # Run the application
xplat task check      # Run fmt, lint, test
```

## Structure

```
plat-ibkr/
├── .src/             # Vendored source (optional ibapi clone)
├── .bin/             # Built binaries
├── src/              # Project source code
├── Cargo.toml        # Rust dependencies
├── Taskfile.yml      # Task runner
├── pc.yaml           # Process Compose config
└── xplat.yaml        # Package manifest
```

## Key Tasks

```sh
# Build
task build            # Debug build
task build:release    # Release build

# Run
task run              # Run debug
task run:release      # Run release
task dev              # Run with auto-reload (cargo watch)

# Test & Quality
task test             # Run tests
task fmt              # Format code
task lint             # Run clippy
task check            # All checks (fmt, lint, test)

# xplat Generation
task gen              # Generate all xplat files
task gen:workflow     # Generate CI workflow
task bootstrap        # Bootstrap standard plat-* files

# Dependencies
task ibapi:deps:clone # Clone ibapi source to .src/
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| RUST_LOG | info | Logging level |
| RUST_BACKTRACE | 1 | Enable backtraces |
| IBKR_HOST | 127.0.0.1 | TWS/Gateway host |
| IBKR_PORT | 4001 | TWS/Gateway port |

## IBKR Setup

Requires TWS or IB Gateway running:
- TWS: Enable API in Configuration > API > Settings
- Gateway: Port 4001 (live) or 4002 (paper)

## CI

GitHub Actions workflow at `.github/workflows/ci.yml`:
- Runs on Linux, macOS, Windows
- Uses xplat for cross-platform execution
- Runs: `task build`, `task test`, `task lint`

Regenerate with: `task gen:workflow`
