use clap::{Parser, ValueEnum};
use csv::Writer;
use ibapi::accounts::{
    types::AccountGroup, AccountSummaryResult, AccountSummaryTags, PositionUpdate,
};
use ibapi::client::blocking::Client;
use ibapi::prelude::*;
use serde::Serialize;
use std::env;
use std::io;

#[derive(Parser)]
#[command(name = "plat-ibkr")]
#[command(about = "IBKR trading platform CLI")]
struct Cli {
    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    format: OutputFormat,

    /// Symbol for market data (default: AAPL)
    #[arg(short, long, default_value = "AAPL")]
    symbol: String,

    /// Skip market data request
    #[arg(long)]
    no_market_data: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}

#[derive(Serialize)]
struct AccountSummaryRow {
    account: String,
    tag: String,
    value: String,
    currency: String,
}

#[derive(Serialize)]
struct PositionRow {
    account: String,
    symbol: String,
    position: f64,
    average_cost: f64,
    market_value: f64,
}

#[derive(Serialize)]
struct MarketDataRow {
    symbol: String,
    tick_type: String,
    value: f64,
}

#[derive(Serialize)]
struct OutputData {
    account_summary: Vec<AccountSummaryRow>,
    positions: Vec<PositionRow>,
    market_data: Vec<MarketDataRow>,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let host = env::var("IBKR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("IBKR_PORT").unwrap_or_else(|_| "4002".to_string());
    let connection_url = format!("{}:{}", host, port);

    if cli.format == OutputFormat::Text {
        eprintln!("Connecting to TWS/Gateway at {}...", connection_url);
    }

    let client = match Client::connect(&connection_url, 100) {
        Ok(c) => {
            if cli.format == OutputFormat::Text {
                eprintln!("Connected successfully!\n");
            }
            c
        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
            eprintln!("\nMake sure TWS or IB Gateway is running with API enabled:");
            eprintln!("  - TWS: Configure > API > Settings > Enable ActiveX and Socket Clients");
            eprintln!("  - Gateway: Port 4001 (live) or 4002 (paper)");
            std::process::exit(1);
        }
    };

    let mut data = OutputData {
        account_summary: Vec::new(),
        positions: Vec::new(),
        market_data: Vec::new(),
    };

    // Collect account summary
    collect_account_summary(&client, &mut data.account_summary);

    // Collect positions
    collect_positions(&client, &mut data.positions);

    // Collect market data
    if !cli.no_market_data {
        collect_market_data(&client, &cli.symbol, &mut data.market_data);
    }

    // Output based on format
    match cli.format {
        OutputFormat::Text => print_text(&data, &cli.symbol),
        OutputFormat::Json => print_json(&data),
        OutputFormat::Csv => print_csv(&data),
    }
}

fn collect_account_summary(client: &Client, rows: &mut Vec<AccountSummaryRow>) {
    let tags = &[
        AccountSummaryTags::ACCOUNT_TYPE,
        AccountSummaryTags::NET_LIQUIDATION,
        AccountSummaryTags::TOTAL_CASH_VALUE,
        AccountSummaryTags::BUYING_POWER,
        AccountSummaryTags::GROSS_POSITION_VALUE,
        AccountSummaryTags::AVAILABLE_FUNDS,
    ];

    if let Ok(subscription) = client.account_summary(&AccountGroup("All".to_string()), tags) {
        for update in &subscription {
            match update {
                AccountSummaryResult::Summary(summary) => {
                    rows.push(AccountSummaryRow {
                        account: summary.account.clone(),
                        tag: summary.tag.clone(),
                        value: summary.value.clone(),
                        currency: summary.currency.clone(),
                    });
                }
                AccountSummaryResult::End => {
                    subscription.cancel();
                    break;
                }
            }
        }
    }
}

fn collect_positions(client: &Client, rows: &mut Vec<PositionRow>) {
    if let Ok(positions) = client.positions() {
        while let Some(update) = positions.next() {
            match update {
                PositionUpdate::Position(pos) => {
                    rows.push(PositionRow {
                        account: pos.account.clone(),
                        symbol: pos.contract.symbol.to_string(),
                        position: pos.position,
                        average_cost: pos.average_cost,
                        market_value: pos.position * pos.average_cost,
                    });
                }
                PositionUpdate::PositionEnd => {
                    positions.cancel();
                    break;
                }
            }
        }
    }
}

fn collect_market_data(client: &Client, symbol: &str, rows: &mut Vec<MarketDataRow>) {
    let contract = Contract::stock(symbol).build();

    if let Ok(subscription) = client.market_data(&contract).snapshot().subscribe() {
        for tick in &subscription {
            match tick {
                TickTypes::Price(price) => {
                    rows.push(MarketDataRow {
                        symbol: symbol.to_string(),
                        tick_type: format!("{:?}", price.tick_type),
                        value: price.price,
                    });
                }
                TickTypes::Size(size) => {
                    rows.push(MarketDataRow {
                        symbol: symbol.to_string(),
                        tick_type: format!("{:?}", size.tick_type),
                        value: size.size,
                    });
                }
                TickTypes::PriceSize(ps) => {
                    rows.push(MarketDataRow {
                        symbol: symbol.to_string(),
                        tick_type: format!("{:?}", ps.price_tick_type),
                        value: ps.price,
                    });
                }
                TickTypes::SnapshotEnd => break,
                _ => {}
            }
        }
    }
}

fn print_text(data: &OutputData, symbol: &str) {
    println!("{}", "=".repeat(50));
    println!("ACCOUNT SUMMARY");
    println!("{}", "=".repeat(50));
    for row in &data.account_summary {
        if row.currency.is_empty() {
            println!("  {}: {} = {}", row.account, row.tag, row.value);
        } else {
            println!(
                "  {}: {} = {} {}",
                row.account, row.tag, row.value, row.currency
            );
        }
    }
    println!();

    println!("{}", "=".repeat(50));
    println!("POSITIONS");
    println!("{}", "=".repeat(50));
    if data.positions.is_empty() {
        println!("  (no positions)");
    } else {
        for row in &data.positions {
            println!(
                "  {:>8.2} {} @ ${:.2} avg (value: ${:.2})",
                row.position, row.symbol, row.average_cost, row.market_value
            );
        }
    }
    println!();

    println!("{}", "=".repeat(50));
    println!("MARKET DATA: {}", symbol);
    println!("{}", "=".repeat(50));
    if data.market_data.is_empty() {
        println!("  (no data - may need market data subscription)");
    } else {
        for row in &data.market_data {
            println!("  {}: {:.2}", row.tick_type, row.value);
        }
    }
    println!();
    println!("Done!");
}

fn print_json(data: &OutputData) {
    println!("{}", serde_json::to_string_pretty(data).unwrap());
}

fn print_csv(data: &OutputData) {
    // Account summary
    if !data.account_summary.is_empty() {
        eprintln!("# Account Summary");
        let mut wtr = Writer::from_writer(io::stdout());
        for row in &data.account_summary {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
        println!();
    }

    // Positions
    if !data.positions.is_empty() {
        eprintln!("# Positions");
        let mut wtr = Writer::from_writer(io::stdout());
        for row in &data.positions {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
        println!();
    }

    // Market data
    if !data.market_data.is_empty() {
        eprintln!("# Market Data");
        let mut wtr = Writer::from_writer(io::stdout());
        for row in &data.market_data {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
    }
}
