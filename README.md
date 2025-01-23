# Axum SQLx Experimental

### To start application
```sh
cargo run
```
### For watch mode
Before start application with watch mode you need to install `cargo-watch` first.
```sh
cargo install cargo-watch
```
Start application with watch mode
```sh
cargo watch -c -w src -x run
```

### List of Dependencies

```sh
cargo add axum
cargo add tokio -F full
cargo add tower-http -F "cors"
cargo add serde_json
cargo add serde -F derive
cargo add chrono -F serde
cargo add dotenv
cargo add uuid -F "serde v4"
cargo add sqlx --features "runtime-async-std-native-tls mysql chrono uuid"
```