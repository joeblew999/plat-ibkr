# plat-ibkr

IBKR (Interactive Brokers) trading platform using [rust-ibapi](https://github.com/wboayue/rust-ibapi).

- [IBKR API docs](https://ibkrcampus.com/campus/ibkr-api-page/twsapi-doc/)

## Prerequisites

- [xplat](https://github.com/joeblew999/xplat) (bundles Task + process-compose)
- Rust 1.75+
- Docker (for IB Gateway)
- IBKR account with API access

## Quick Start

```bash
# 1. Setup
task setup

# 2. Configure credentials
cp .env.example .env
# Edit .env with your IBKR credentials

# 3. Start IB Gateway (Docker)
task gateway:docker

# 4. Build and run
task build
task run
```

## Getting IBKR Credentials

### New Account (Paper Trading - Free)

1. Go to [interactivebrokers.com](https://www.interactivebrokers.com)
2. Click **Open Account** > **Individual**
3. Enable **Paper Trading Account** during signup
4. Once approved, use your IBKR username/password

### Existing Account

Enable paper trading in Account Management:
- Login > Settings > Paper Trading Account > Enable

## Configuration

Copy `.env.example` to `.env`:

```bash
cp .env.example .env
```

Edit `.env`:

```env
# Required for Docker gateway
TWS_USERID=your_ibkr_username
TWS_PASSWORD=your_ibkr_password

# App settings (defaults shown)
IBKR_HOST=127.0.0.1
IBKR_PORT=4002          # 4002=paper, 4001=live
RUST_LOG=info
```

## Tasks

### Common Commands

| Command | Description |
|---------|-------------|
| `task` | List all tasks |
| `task build` | Build the project |
| `task run` | Run the application |
| `task test` | Run tests |
| `task check` | Run fmt, lint, and tests |
| `task setup` | Setup development environment |

### IB Gateway

| Command | Description |
|---------|-------------|
| `task gateway:docker` | Start IB Gateway (paper trading) |
| `task gateway:docker:live` | Start IB Gateway (LIVE - careful!) |
| `task gateway:docker:stop` | Stop IB Gateway |
| `task gateway:docker:status` | Check gateway status |
| `task gateway:docker:logs` | View gateway logs |
| `task gateway:docker:vnc` | Open VNC to gateway (macOS) |

### Development

| Command | Description |
|---------|-------------|
| `task rust:dev` | Run with auto-reload |
| `task rust:fmt` | Format code |
| `task rust:lint` | Run clippy |
| `task rust:test:watch` | Run tests in watch mode |
| `task rust:docs` | Generate and open docs |

### Binary Management

| Command | Description |
|---------|-------------|
| `task bin:build` | Build release binary to `.bin/` |
| `task bin:run` | Run release binary |

## Project Structure

```
plat-ibkr/
├── src/
│   └── main.rs          # Application entry point
├── .bin/                # Release binaries (gitignored)
├── .src/                # Cloned dependencies (gitignored)
├── .data/               # Runtime data (gitignored)
├── .gateway/            # Gateway installers (gitignored)
├── Cargo.toml           # Rust dependencies
├── Taskfile.yml         # Task definitions
├── pc.yaml              # Process compose config
├── xplat.yaml           # xplat manifest
├── .env.example         # Environment template
└── README.md
```

## Ports

| Port | Mode | Description |
|------|------|-------------|
| 4001 | Live | Live trading API |
| 4002 | Paper | Paper trading API (default) |
| 5900 | VNC | Gateway GUI access |

## Process Compose

Run everything with process-compose:

```bash
# Start gateway + app
process-compose up

# Or with xplat
xplat pc up
```

## License

MIT
