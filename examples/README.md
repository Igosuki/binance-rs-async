# Usage

## Binance Endpoints

cargo run --release --example "binance_endpoints"

## Binance Websockets

cargo run --release --example "binance_websockets"

## Binance Websockets - Save all trades to file

cargo run --release --example "binance_save_all_trades"

tmux new -s <session name>

tmux a -t <session id>

rsync target/x86_64-unknown-linux-gnu/release/examples/binance_websockets binance:~/

cross build --release --example "binance_websockets" --target x86_64-unknown-linux-gnu