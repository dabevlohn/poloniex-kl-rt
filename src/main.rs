//! MVP for Poloniex Websocket API
//!
//! This is a work in progress and not intended for production use.
//!
//! See https://api-docs.poloniex.com/spot/websocket/

use duckdb::Connection;
use serde::{Deserialize, Serialize};
use tungstenite::{connect, Message};

/// A recent trade structure with native param names
#[derive(Serialize, Deserialize, Debug)]
struct RecentTrade {
    id: String,
    symbol: String,
    price: String,
    amount: String,
    #[serde(rename = "takerSide")]
    takerside: String,
    ts: i64,
    quantity: String,
    #[serde(rename = "createTime")]
    createtime: i64,
}

/// A structure for the channel trades message
#[derive(Serialize, Deserialize, Debug)]
struct ChannelTrades {
    channel: String,
    data: Vec<RecentTrade>,
}

fn main() {
    let (mut socket, _) = connect("wss://ws.poloniex.com/ws/public").expect("Failed to connect");

    //  Can subscribe to few channels in one message
    //  "channel": ["candles_minute_1", "trades"],
    //  but can't parse all channel messages in one stream
    socket
        .send(Message::text(
            r#"{
      "event": "subscribe",
      "channel": ["trades"],
      "symbols": ["BTC_USDT", "BCH_USDT", "TRX_USDT", "ETH_USDT", "DOGE_USDT"]
    }"#,
        ))
        .unwrap();

    // Prepare duckdb connection
    let conn: Connection = Connection::open_in_memory().expect("Failed to open db");
    conn.execute_batch(
        r"CREATE TABLE trades (
              id VARCHAR,
              takerside VARCHAR,
              symbol VARCHAR,
              price VARCHAR,
              amount VARCHAR,
              quantity VARCHAR,
              ts VARCHAR,
              createtime VARCHAR
        );",
    )
    .expect("Failed to create table");
    let mut app = conn.appender("trades").unwrap();
    let mut counter = 0;

    loop {
        let msg = socket.read().expect("Error reading message");
        let text_str = msg.to_text().unwrap();
        if text_str.contains("event") {
            println!("Start receiving trades...");
        } else {
            let data: ChannelTrades = serde_json::from_str(text_str).unwrap();
            match app.append_row([
                data.data[0].id.clone().to_string(),
                data.data[0].takerside.clone().to_string(),
                data.data[0].symbol.clone().to_string(),
                data.data[0].price.clone().to_string(),
                data.data[0].amount.clone().to_string(),
                data.data[0].quantity.clone().to_string(),
                data.data[0].ts.clone().to_string(),
                data.data[0].createtime.clone().to_string(),
            ]) {
                Ok(_) => {
                    counter += 1;
                    println!("{} Row appended", counter);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    break;
                }
            }
        }
        if counter >= 100 {
            app.flush().unwrap();
            conn.execute_batch(
                r"COPY (SELECT * FROM trades)
                TO 'trades.parquet'
                (FORMAT PARQUET, COMPRESSION ZSTD);
                ",
            )
            .expect("Failed to save db file");
            break;
        }
    }
}
