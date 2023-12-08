# ToyDB Web Client

Web frontend for [ToyDB](https://github.com/dowlandaiello/toydb) written by [@dowlandaiello](https://github.com/dowlandaiello).

## Run
First, install [Trunk](https://trunkrs.dev/):
```
cargo install trunk
```

Then, add a wasm target:
```
rustup target add wasm32-unknown-unknown
```

Finally, compile the code and start the server with:
```
cargo build
trunk serve
```

## Config

Configuration options are available in `Trunk.toml`. The `ToyDB` API endpoint can be configured in `lib.rs`
