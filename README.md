# Experiments with Poloniex public API

No API Keys needed

## Prerequisites

```bash
# Install some tools
cargo install hurl bacon just
# Check source code
bacon
# Check Poloniex public API
just polo_api
# Run app
cargo run
```

## How it works

This app subscribes to websocket channel and receives messages about recent
trades. Each 100 messages will be saved to Parquet-formatted file.

## How to read Parquet file

```bash
just read_data
```
