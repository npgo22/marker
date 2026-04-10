# chunk_convert (Rust)

Experimental Rust port of the chunk conversion launcher logic from `marker/scripts/chunk_convert.sh`.

## Build

```bash
cd rust/chunk_convert
cargo build --release
```

## Run

Set `NUM_DEVICES` and `NUM_WORKERS`, then run with input and output folders:

```bash
NUM_DEVICES=2 NUM_WORKERS=8 ./target/release/chunk_convert /path/to/input /path/to/output
```
