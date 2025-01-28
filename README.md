# WIP! Experiments with Poloniex public API

Only MVP is implemented, but it works.
No API Keys needed.

## Planned features

- [x] Get Candles for few pairs by public Poloniex REST API
- [x] Subscribe to public Poloniex websocket channels Trades and Candles
- [x] Parse incoming messages
- [x] Save data to Parquet files
- [x] Read data from Parquet files
- [ ] Save Parquet files to S3 buckets
- [ ] Ingest data to QuickWit Index
- [ ] Query data from QuickWit Index with Grafana
- [ ] Config file for starting app
- [ ] Socket for getting new parameters and locking app instance
- [ ] Thread pool for websocket loops
- [ ] IPC for communication between instances of the same app
- [ ] Helm Chart for deploying app in Kubernetes

## Prerequisites

```bash
# Install some tools
cargo install hurl bacon just
cargo install parquet --features="cli,arrow,zstd,json,async"
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
