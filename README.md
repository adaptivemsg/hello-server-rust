# hello-server-rust

## Generate Go bindings

This repo generates `api/hello/message.go` and `go.mod` from `api/hello/message.rs` using `amgen-rs`.

Install the generator:

```bash
cargo install adaptivemsg-amgen
#cargo install --path ../adaptivemsg-rust/cmd/amgen/
```

Run generation:

```bash
~/.cargo/bin/amgen-rs --in api/hello/message.rs
```

## Beginner how-to

### Prerequisites

- Rust installed (`cargo version`)
- Go installed only if you want to run the Go client (`go version`)

### Generate Go bindings (optional)

```bash
~/.cargo/bin/amgen-rs --in api/hello/message.rs
```

### Run the server

```bash
cargo run --bin server
```

### Try a client

Go client:

```bash
(cd ../hello-client-go && go run .)
```

Rust client:

```bash
(cd ../hello-client-rust && cargo run)
```

### Common issues

- `amgen-rs` not found: install with `cargo install adaptivemsg-amgen` or `cargo install --path ../adaptivemsg-rust/cmd/amgen`
