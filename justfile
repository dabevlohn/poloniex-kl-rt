check := "cargo check --color always"
clippy := "cargo clippy --color always"
args := "--color always"
clippy_w := "-W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used"
clippy_a := "-A clippy::missing-errors-doc -A clippy::missing-panics-doc"
clippy_args := "-- " + clippy_w + " " + clippy_a

# For bacon
clippy:
    cargo clippy {{args}} {{clippy_args}}

# Check only
check:
    cargo check {{args}}

# Check Poloniex Candles API
polo_api:
    hurl check_candles_api.hurl

# Read saved data
read_data:
    #duckdb -c 'select * from trades.parquet;'
    parquet-read trades.parquet
